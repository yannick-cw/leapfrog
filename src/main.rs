use std::{
    convert::Infallible,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::{Arg, Parser, Subcommand};
mod errors;
mod jwt;
mod leapsome_api;
use crate::{
    errors::AppErr,
    jwt::{
        decode_jwt::decode_jwt,
        token_keyring::{read_tkn, store_tkn},
    },
    leapsome_api::{LeapfrogApi, LeapsomeApi, leapsome_req_res::LeapsomeClient},
};


#[derive(Parser, Debug)]
#[command(version, about = "Leapsome CLI for managing feedback and goals. Tokens auto-refresh for 31 days after initial login.")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// One-time setup: store tokens from a dedicated browser profile.
    #[command(long_about = "One-time setup: store tokens from a DEDICATED browser profile.\n\nWARNING: You MUST use a separate browser profile (not your daily one)!\nIf you open Leapsome in the same browser profile, the CLI tokens are\ninvalidated immediately and you have to re-login.\n\nSteps:\n  1) Create a new browser profile (e.g. Firefox/Chrome profile) solely for CLI tokens\n  2) Log into Leapsome in that profile\n  3) Open DevTools > Network, find any API request\n  4) Copy the Authorization header value (without 'Bearer ')\n  5) Copy the l_refresh_token cookie value from the Cookie REQUEST HEADER\n     (not Application > Cookies). The value must start with s%3A.\n     If yours starts with s: you grabbed the decoded version;\n     replace s: with s%3A before pasting.\n  6) Run: leapfrog login <access_token> <refresh_token>\n  7) Never use Leapsome in that browser profile again\n\nThe CLI auto-refreshes tokens for up to 31 days after initial login.")]
    Login {
        /// The Bearer token from the Authorization header
        token: Token,
        /// The l_refresh_token cookie value (must start with s%3A, not s:)
        refresh_token: RefreshToken,
    },
    /// Manage your Leapsome goals/priorities
    #[command(subcommand)]
    Goals(GoalsCommand),
    /// Search Leapsome users
    #[command(subcommand)]
    Users(UserCommand),
    /// View and send instant feedback
    #[command(subcommand)]
    Feedback(FeedbackCommand),
}

#[derive(Debug, Subcommand)]
enum GoalsCommand {
    /// List your goals and your team's goals
    List,
    /// Post a comment on a goal or key result
    Comment {
        #[arg(long, short, help = "The goal ID to comment on")]
        goal_id: String,
        #[arg(long, short = 'x', help = "The comment text")]
        content: String,
        #[arg(long, short = 'r', help = "Optional key result ID")]
        key_result_id: Option<String>,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum FeedbackType {
    Praise,
    Structured,
    PrivateNote,
}
#[derive(Debug, Clone, clap::ValueEnum)]
enum Visibility {
    Receiver,
    ReceiverManager,
}

#[derive(Debug, Subcommand)]
enum FeedbackCommand {
    /// List received feedback
    List {
        #[arg(long, short, help = "Filter by feedback type")]
        kind: Option<FeedbackType>,
    },
    /// Send praise to a colleague
    Praise {
        #[arg(long, short, help = "Leapsome user ID (find via 'users search')")]
        user_id: String,
        #[arg(long, short, help = "The praise message")]
        content: String,
        #[arg(long, short, default_value = "receiver-manager", help = "Who can see this feedback")]
        visibility: Visibility,
    },
}

#[derive(Debug, Subcommand)]
#[command(args([Arg::new("search")]))]
enum UserCommand {
    /// Find a user by name
    Search {
        /// Name or partial name to search for
        name: String,
    },
}


#[derive(Debug, Clone)]
struct Token(String);
impl FromStr for Token {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Token(s.to_string()))
    }
}

#[derive(Debug, Clone)]
struct RefreshToken(String);

impl FromStr for RefreshToken {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RefreshToken(s.to_string()))
    }
}

#[derive(Debug)]
struct Env {
    token: Option<(Token, RefreshToken)>,
}
fn read_env() -> Result<Env, AppErr> {
    read_tkn()
        .inspect_err(|e| println!("Failed reading token file {:?}", e))
        .map(|(tkn, refresh)| {
            Ok(Env {
                token: Some((tkn, refresh)),
            })
        })
        .unwrap_or_else(|_| Ok(Env { token: None }))
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

async fn leapsome_api() -> Result<LeapsomeApi<LeapsomeClient>, AppErr> {
    let (tkn, refresh) = read_env().and_then(|env| match env.token {
        Some((tkn, refresh)) => Ok((tkn, refresh)),
        None => Err(AppErr::EnvErr(
            "No token in config file found, try 'login'".to_string(),
        )),
    })?;
    // println!("Env ready {:?}!", tkn);
    let info = decode_jwt(&tkn)?;
    let expiry_date = info.exp;

    let client = LeapsomeClient {
        token: tkn,
        refresh_token: refresh,
        user_info: info,
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // why: token refresh needed
    let updated_client = if expiry_date < now {
        println!("Going for token refresh");
        let (tkn, refresh) = client.update_token().await?;

        store_tkn(&tkn, &refresh)?;
        let info = decode_jwt(&tkn)?;

        LeapsomeClient {
            token: tkn,
            refresh_token: refresh,
            user_info: info,
        }
    } else {
        client
    };

    let api = LeapsomeApi {
        client: updated_client,
    };
    Ok(api)
}

async fn run() -> Result<(), AppErr> {
    let cli = Args::parse();
    match cli.command {
        Command::Login {
            token,
            refresh_token,
        } => {
            store_tkn(&token, &refresh_token)?;
            println!("Tokens stored! The CLI will auto-refresh for up to 31 days.");
        }
        Command::Goals(GoalsCommand::List) => {
            let api = leapsome_api().await?;
            let goals = api.list_goals().await?;
            println!("{}", serde_json::to_string_pretty(&goals).unwrap());
        }
        Command::Goals(GoalsCommand::Comment {
            goal_id,
            content,
            key_result_id,
        }) => {
            let api = leapsome_api().await?;
            let comment = api
                .post_goal_comment(&content, &goal_id, key_result_id.as_deref())
                .await?;
            println!("{}", serde_json::to_string_pretty(&comment).unwrap());
        }
        Command::Feedback(FeedbackCommand::List { kind }) => {
            let api = leapsome_api().await?;
            let feedback = api.all_feedback(kind).await?;
            println!("{}", serde_json::to_string_pretty(&feedback).unwrap());
        }
        Command::Feedback(FeedbackCommand::Praise {
            user_id,
            content,
            visibility,
        }) => {
            let api = leapsome_api().await?;
            let feedback = api
                .post_feedback(&content, &user_id, visibility.into())
                .await?;
            println!("{}", serde_json::to_string_pretty(&feedback).unwrap());
        }
        Command::Users(UserCommand::Search { name }) => {
            let api = leapsome_api().await?;
            let found_users = api.search_leapsome_user(&name).await?;
            println!("{}", serde_json::to_string_pretty(&found_users).unwrap());
        }
    };
    Ok(())
}
