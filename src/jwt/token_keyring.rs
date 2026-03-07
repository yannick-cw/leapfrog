use crate::{RefreshToken, Token, errors::AppErr};

const SERVICE: &str = "leapfrog-cli";
const TOKEN_PATH: &str = "access_token";
const REFRESH_TOKEN_PATH: &str = "refresh_token";

pub fn read_tkn() -> Result<(Token, RefreshToken), AppErr> {
    read_tkn_from(&SERVICE)
}
fn read_tkn_from(service: &str) -> Result<(Token, RefreshToken), AppErr> {
    let token_entry = keyring::Entry::new(service, TOKEN_PATH)?;
    let refresh_entry = keyring::Entry::new(service, REFRESH_TOKEN_PATH)?;

    Ok((
        Token(token_entry.get_password()?),
        RefreshToken(refresh_entry.get_password()?),
    ))
}

pub fn store_tkn(tkn: &Token, refresh_tkn: &RefreshToken) -> Result<(), AppErr> {
    store_tkn_to(&tkn, &refresh_tkn, &SERVICE)
}
fn store_tkn_to(tkn: &Token, refresh_tkn: &RefreshToken, service: &str) -> Result<(), AppErr> {
    let token_entry = keyring::Entry::new(service, TOKEN_PATH)?;
    let refresh_entry = keyring::Entry::new(service, REFRESH_TOKEN_PATH)?;

    token_entry.set_password(&tkn.0)?;
    refresh_entry.set_password(&refresh_tkn.0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_read_tokens_keyring() {
        store_tkn_to(
            &Token("ABC".to_string()),
            &RefreshToken("DEF".to_string()),
            "test-leapfrog",
        )
        .unwrap();

        let (token, refresh_token) = read_tkn_from("test-leapfrog").unwrap();
        assert_eq!(token.0, "ABC");
        assert_eq!(refresh_token.0, "DEF")
    }
}
