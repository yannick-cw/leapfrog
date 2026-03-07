use crate::{
    errors::AppErr,
    leapsome_api::leapsome_req_res::{ApiSingleResponse, LeapsomePost},
};

#[derive(Debug, serde::Serialize)]
pub struct FeedbackSignal {
    content: String,
    visibility: Visibility,
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]

pub enum Visibility {
    Receiver,
    ReceiverManager,
}
impl From<crate::Visibility> for Visibility {
    fn from(vis: crate::Visibility) -> Self {
        match vis {
            crate::Visibility::Receiver => Visibility::Receiver,
            crate::Visibility::ReceiverManager => Visibility::ReceiverManager,
        }
    }
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SendFeedback {
    feedback_signals: Vec<FeedbackSignal>,
    multi_receiver: Vec<String>,
    r#type: String,
    visibility: Visibility,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackResponse {
    feedback_set: Option<String>,
}

pub type FeedbackCreateResponse = ApiSingleResponse<FeedbackResponse>;

pub async fn post_feedback<P: LeapsomePost>(
    content: &str,
    receiver: &str,
    visibility: Visibility,
    client: &P,
) -> Result<FeedbackCreateResponse, AppErr> {
    let req: SendFeedback = SendFeedback {
        feedback_signals: vec![FeedbackSignal {
            content: content.to_string(),
            visibility: Visibility::Receiver,
        }],
        multi_receiver: vec![receiver.to_string()],
        visibility: visibility,
        r#type: "thanks-note".to_string(),
    };

    client.post(&req, "/feedback/create").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialise_request() {
        let req: SendFeedback = SendFeedback {
            feedback_signals: vec![FeedbackSignal {
                content: "Feedback".to_string(),
                visibility: Visibility::Receiver,
            }],
            multi_receiver: vec!["receiver".to_string()],
            visibility: Visibility::Receiver,
            r#type: "thanks-note".to_string(),
        };
        let expected: serde_json::Value = serde_json::from_str(
            r#"
            {
                "feedbackSignals": [{
                    "content": "Feedback",
                    "visibility": "receiver"
                }],
                "multiReceiver": ["receiver"],
                "type": "thanks-note",
                "visibility": "receiver"
            }
            "#,
        )
        .unwrap();
        let actual: serde_json::Value = serde_json::to_value(&req).unwrap();
        assert_eq!(actual, expected);
    }
}
