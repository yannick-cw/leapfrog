use reqwest::{
    Response, StatusCode,
    header::{self, AUTHORIZATION, HeaderMap, HeaderName, HeaderValue},
};

use crate::{RefreshToken, Token, errors::AppErr, jwt::decode_jwt::UserInfo};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiListResponse<Data> {
    data: Vec<Data>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ApiSingleResponse<Data> {
    data: Data,
}

pub trait LeapsomePost {
    async fn post<Req, Res>(&self, req: &Req, path: &str) -> Result<Res, AppErr>
    where
        Req: serde::Serialize + std::fmt::Debug,
        Res: serde::de::DeserializeOwned + std::fmt::Debug;
}
pub struct LeapsomeClient {
    pub token: Token,
    pub refresh_token: RefreshToken,
    pub user_info: UserInfo,
}

const LEAPSOME_URL: &str = "https://www.leapsome.com/api";

#[derive(Debug, serde::Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

impl LeapsomeClient {
    fn base_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token.0)).expect("Nope"),
        );
        headers.insert(
            HeaderName::from_static("x-leapsome-user-id"),
            HeaderValue::from_str(&self.user_info._id).expect("Nope"),
        );
        headers.insert(
            HeaderName::from_static("x-leapsome-tenant-id"),
            HeaderValue::from_str(&self.user_info.team_role.team).expect("Nope"),
        );
        headers
    }

    async fn post_raw<Req: serde::Serialize>(
        &self,
        path: &str,
        extra_headers: Option<HeaderMap>,
        req: &Req,
    ) -> Result<Response, AppErr> {
        let mut headers = self.base_headers();
        if let Some(extra) = extra_headers {
            headers.extend(extra);
        }

        let raw_response = reqwest::Client::new()
            .post(format!("{LEAPSOME_URL}{path}"))
            .headers(headers)
            .json(&req)
            .send()
            .await?;

        // println!("Received: {:?}", raw_response);

        Ok(raw_response)
    }

    pub async fn update_token(&self) -> Result<(Token, RefreshToken), AppErr> {
        let mut cookie_header = HeaderMap::new();
        cookie_header.insert(
            header::COOKIE,
            HeaderValue::from_str(&format!("l_refresh_token={}", self.refresh_token.0))
                .expect("Nope"),
        );

        #[derive(serde::Serialize)]
        struct TzBody {
            tz: String,
        }

        let response = self
            .post_raw(
                "/users/update/token",
                Some(cookie_header),
                &TzBody {
                    tz: "Europe/Berlin".to_string(),
                },
            )
            .await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(AppErr::ApiReqErr(
                "Refresh token expired or rotated. Grab fresh tokens from your dedicated browser profile and run: leapfrog login <access_token> <refresh_token>".to_string(),
            ));
        }

        let set_cookie = response
            .headers()
            .get_all(header::SET_COOKIE)
            .iter()
            .find_map(|val| {
                let s = val.to_str().ok()?;
                if s.starts_with("l_refresh_token=") {
                    s.split(';')
                        .next()
                        .map(|v| v.trim_start_matches("l_refresh_token=").to_string())
                } else {
                    None
                }
            })
            .ok_or(AppErr::ApiReqErr(
                "No refresh token in response".to_string(),
            ))?;

        let body: TokenResponse = response.json().await?;

        Ok((Token(body.token), RefreshToken(set_cookie)))
    }
}

impl LeapsomePost for LeapsomeClient {
    async fn post<Req, Res>(&self, req: &Req, path: &str) -> Result<Res, AppErr>
    where
        Req: serde::Serialize + std::fmt::Debug,
        Res: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let response = self.post_raw(path, None, req).await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(AppErr::ApiReqErr(
                "Not authorized. Try: leapfrog login <access_token> <refresh_token>".to_string(),
            ));
        }

        let decoded_response: Res = response.json().await?;

        // println!("Parsed to: {:?}", decoded_response);

        Ok(decoded_response)
    }
}
