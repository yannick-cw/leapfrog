use crate::{
    FeedbackType,
    errors::AppErr,
    leapsome_api::leapsome_req_res::{ApiListResponse, LeapsomePost},
};

#[derive(Debug, serde::Serialize)]
pub struct FeedbackInclude {
    _id: String,
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackFilters {
    feedback_type_group: Vec<FeedbackInclude>,
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllFeedbackRequest {
    filters: FeedbackFilters,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    _id: Option<String>,
    displayed_name: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackResponse {
    #[serde(rename = "_id")]
    _id: Option<String>,
    content: Option<String>,
    sender: Option<User>,
    receiver: Option<User>,
    thanks_note_receivers: Option<Vec<User>>,
}

pub type AllFeedbackList = ApiListResponse<FeedbackResponse>;

pub async fn all_feedback<P: LeapsomePost>(
    kind: Option<FeedbackType>,
    client: &P,
) -> Result<AllFeedbackList, AppErr> {
    let feedback_filter = match kind {
        Some(FeedbackType::Praise) => vec![FeedbackInclude {
            _id: "praise".to_string(),
        }],
        Some(FeedbackType::Structured) => vec![FeedbackInclude {
            _id: "instantFeedback".to_string(),
        }],
        Some(FeedbackType::PrivateNote) => vec![FeedbackInclude {
            _id: "privateNote".to_string(),
        }],
        None => vec![],
    };
    let req: AllFeedbackRequest = AllFeedbackRequest {
        filters: FeedbackFilters {
            feedback_type_group: feedback_filter,
        },
    };

    client
        .post(&req, "/feedback/get/user/all/sets/all-filtered")
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialise_request() {
        let req: AllFeedbackRequest = AllFeedbackRequest {
            filters: FeedbackFilters {
                feedback_type_group: vec![FeedbackInclude {
                    _id: "praise".to_string(),
                }],
            },
        };
        let expected: serde_json::Value = serde_json::from_str(
            r#"
            {
                "filters": {
                    "feedbackTypeGroup": [
                        {
                            "_id": "praise"
                        }
                    ]
                }
            }
            "#,
        )
        .unwrap();
        let actual: serde_json::Value = serde_json::to_value(&req).unwrap();
        assert_eq!(actual, expected);
    }
}
