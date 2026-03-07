use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppErr {
    #[error("env missing: {0}")]
    EnvErr(String),
    #[error("failed talking to leapsome: {0}")]
    ApiReqErr(String),
    #[error("failed decoding token: {0}")]
    TokenDecodeErr(String),
    #[error("failed IO token: {0}")]
    TokenIOErr(String),
}
impl From<std::io::Error> for AppErr {
    fn from(err: std::io::Error) -> Self {
        AppErr::TokenIOErr(format!("{err:?}"))
    }
}
impl From<reqwest::Error> for AppErr {
    fn from(err: reqwest::Error) -> Self {
        AppErr::ApiReqErr(format!("{err:?}"))
    }
}
impl From<base64::DecodeError> for AppErr {
    fn from(err: base64::DecodeError) -> Self {
        AppErr::TokenDecodeErr(format!("{err:?}"))
    }
}
impl From<keyring::Error> for AppErr {
    fn from(err: keyring::Error) -> Self {
        AppErr::TokenDecodeErr(format!("{err:?}"))
    }
}
impl From<serde_json::Error> for AppErr {
    fn from(err: serde_json::Error) -> Self {
        AppErr::TokenDecodeErr(format!("{err:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_err_renders_msg() {
        let err = AppErr::EnvErr("FIELD_X".to_owned());
        assert_eq!(err.to_string(), "env missing: FIELD_X")
    }

    #[test]
    fn api_err_renders_msg() {
        let err = AppErr::ApiReqErr(String::from("Boom!"));
        assert_eq!(err.to_string(), "failed talking to leapsome: Boom!")
    }
}
