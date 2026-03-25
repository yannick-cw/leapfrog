# leapfrog

Unofficial CLI for Leapsome. Manage goals, feedback, and search users from the terminal. JSON output for piping and agent consumption.

**This is a learning project** for me to learn Rust, code is handwritten, AI was used as Tutor.

## Install

```bash
cargo install --path .
```

## Setup

One-time: grab tokens from a **dedicated browser profile** (don't use your daily browser, or token rotation will invalidate the CLI session).

1. Open Leapsome in the dedicated profile
2. DevTools > Network > find any API request
3. Copy the `Authorization` header value (without `Bearer `)
4. Copy the `l_refresh_token` cookie value from the **Cookie request header** (not the Application > Cookies panel).
   The value must start with `s%3A`. If yours starts with `s:` you grabbed the decoded version and need to replace `s:` with `s%3A`.
5. Run:

```bash
leapfrog login <access_token> <refresh_token>
```

Tokens auto-refresh for up to 31 days.

## Usage

```bash
# Goals
leapfrog goals list
leapfrog goals comment --goal-id <ID> --content "On track"
leapfrog goals comment --goal-id <ID> --key-result-id <KR_ID> --content "Done"

# Feedback
leapfrog feedback list
leapfrog feedback list --kind praise
leapfrog feedback praise --user-id <ID> --content "Great work!"

# Users
leapfrog users search "Jane"
```

## Agent usage

All commands output JSON to stdout. Errors go to stderr.

```bash
# Find a user, then send praise
USER_ID=$(leapfrog users search "Jane" | jq -r '.data[0]._id')
leapfrog feedback praise --user-id "$USER_ID" --content "Thanks for the review"

# List goals and comment on the first one
GOAL_ID=$(leapfrog goals list | jq -r '.data.map | to_entries[0].value[0]._id')
leapfrog goals comment --goal-id "$GOAL_ID" --content "Progress update"
```
