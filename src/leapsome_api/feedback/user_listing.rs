use crate::{
    errors::AppErr,
    leapsome_api::leapsome_req_res::{ApiListResponse, LeapsomePost},
};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncludeUserFilter {
    displayed_name: String,
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFilter {
    include: IncludeUserFilter,
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchRequest {
    offset: u32,
    user_filter: UserFilter,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchResponse {
    #[serde(rename = "_id")]
    _id: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    displayed_name: Option<String>,
}

pub type UserResponse = ApiListResponse<UserSearchResponse>;

pub async fn search_leapsome_user<P: LeapsomePost>(
    name: &str,
    client: &P,
) -> Result<UserResponse, AppErr> {
    let req: UserSearchRequest = UserSearchRequest {
        offset: 0,
        user_filter: UserFilter {
            include: IncludeUserFilter {
                displayed_name: name.to_owned(),
            },
        },
    };

    client.post(&req, "/users/list").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialise_request() {
        let req = UserSearchRequest {
            offset: 0,
            user_filter: UserFilter {
                include: IncludeUserFilter {
                    displayed_name: String::from("name"),
                },
            },
        };
        assert_eq!(
            serde_json::to_string(&req).expect("json"),
            "{\"offset\":0,\"userFilter\":{\"include\":{\"displayedName\":\"name\"}}}"
        )
    }
}
