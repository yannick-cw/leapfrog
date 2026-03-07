use crate::errors::AppErr;
use crate::leapsome_api::leapsome_req_res::{ApiSingleResponse, LeapsomePost};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GoalCommentRequest {
    content: String,
    files: Vec<String>,
    goal_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_result_id: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentSender {
    displayed_name: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalComment {
    content: Option<String>,
    sender: Option<CommentSender>,
    goal: Option<String>,
    key_result: Option<String>,
    created_at: Option<String>,
}

pub type GoalCommentResponse = ApiSingleResponse<GoalComment>;

pub async fn post_goal_comment<P: LeapsomePost>(
    content: &str,
    goal_id: &str,
    key_result_id: Option<&str>,
    client: &P,
) -> Result<GoalCommentResponse, AppErr> {
    let req = GoalCommentRequest {
        content: content.to_string(),
        files: vec![],
        goal_id: goal_id.to_string(),
        key_result_id: key_result_id.map(|s| s.to_string()),
    };

    client.post(&req, "/goals/comments").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialise_request_with_key_result() {
        let req = GoalCommentRequest {
            content: "On track".to_string(),
            files: vec![],
            goal_id: "abc123".to_string(),
            key_result_id: Some("kr456".to_string()),
        };
        let expected: serde_json::Value = serde_json::from_str(
            r#"
            {
                "content": "On track",
                "files": [],
                "goalId": "abc123",
                "keyResultId": "kr456"
            }
            "#,
        )
        .unwrap();
        let actual: serde_json::Value = serde_json::to_value(&req).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn serialise_request_without_key_result() {
        let req = GoalCommentRequest {
            content: "On track".to_string(),
            files: vec![],
            goal_id: "abc123".to_string(),
            key_result_id: None,
        };
        let expected: serde_json::Value = serde_json::from_str(
            r#"
            {
                "content": "On track",
                "files": [],
                "goalId": "abc123"
            }
            "#,
        )
        .unwrap();
        let actual: serde_json::Value = serde_json::to_value(&req).unwrap();
        assert_eq!(actual, expected);
    }
}
