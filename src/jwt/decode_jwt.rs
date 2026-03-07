use base64::Engine;

use crate::{Token, errors::AppErr};

#[derive(Debug, serde::Deserialize)]
pub struct TeamRole {
    pub team: String,
}

type ExpiryTimestampSeconds = u64;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[serde(rename = "_id")]
    pub _id: String,
    pub team_role: TeamRole,
    pub exp: ExpiryTimestampSeconds,
}
pub fn decode_jwt(token: &Token) -> Result<UserInfo, AppErr> {
    let payload = token
        .0
        .split(".")
        .enumerate()
        .find(|item| item.0 == 1)
        .map(|item| item.1)
        .ok_or(AppErr::TokenDecodeErr("no body in jwt".to_string()))?;

    let decoded_bytes = base64::engine::general_purpose::STANDARD_NO_PAD.decode(payload)?;

    let decoded_user_info = serde_json::from_slice::<UserInfo>(&decoded_bytes)?;

    Ok(decoded_user_info)
}
