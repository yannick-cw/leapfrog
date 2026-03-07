mod feedback;
mod goals;
pub mod leapsome_req_res;

use crate::{
    FeedbackType, errors::AppErr,
    leapsome_api::feedback::feedback_send::Visibility,
};
use feedback::{
    feedback_listing::{AllFeedbackList, all_feedback},
    feedback_send::{FeedbackCreateResponse, post_feedback},
    user_listing::{UserResponse, search_leapsome_user},
};
use goals::{
    goal_comment::{GoalCommentResponse, post_goal_comment},
    goal_listing::{GoalsListResponse, list_goals},
};
use leapsome_req_res::LeapsomePost;

pub trait LeapfrogApi {
    async fn all_feedback(&self, kind: Option<FeedbackType>) -> Result<AllFeedbackList, AppErr>;
    async fn post_feedback(
        &self,
        content: &str,
        receiver: &str,
        visibility: Visibility,
    ) -> Result<FeedbackCreateResponse, AppErr>;
    async fn search_leapsome_user(&self, name: &str) -> Result<UserResponse, AppErr>;
    async fn list_goals(&self) -> Result<GoalsListResponse, AppErr>;
    async fn post_goal_comment(
        &self,
        content: &str,
        goal_id: &str,
        key_result_id: Option<&str>,
    ) -> Result<GoalCommentResponse, AppErr>;
}
pub struct LeapsomeApi<P: LeapsomePost> {
    pub client: P,
}

impl<P: LeapsomePost> LeapfrogApi for LeapsomeApi<P> {
    async fn all_feedback(&self, kind: Option<FeedbackType>) -> Result<AllFeedbackList, AppErr> {
        all_feedback(kind, &self.client).await
    }

    async fn post_feedback(
        &self,
        content: &str,
        receiver: &str,
        visibility: Visibility,
    ) -> Result<FeedbackCreateResponse, AppErr> {
        post_feedback(&content, &receiver, visibility, &self.client).await
    }

    async fn search_leapsome_user(&self, name: &str) -> Result<UserResponse, AppErr> {
        search_leapsome_user(name, &self.client).await
    }

    async fn list_goals(&self) -> Result<GoalsListResponse, AppErr> {
        list_goals(&self.client).await
    }

    async fn post_goal_comment(
        &self,
        content: &str,
        goal_id: &str,
        key_result_id: Option<&str>,
    ) -> Result<GoalCommentResponse, AppErr> {
        post_goal_comment(content, goal_id, key_result_id, &self.client).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockClient;
    impl LeapsomePost for MockClient {
        async fn post<Req, Res>(&self, _: &Req, _: &str) -> Result<Res, AppErr>
        where
            Req: serde::Serialize + std::fmt::Debug,
            Res: serde::de::DeserializeOwned + std::fmt::Debug,
        {
            Err(AppErr::ApiReqErr("we fail".to_string()))
        }
    }

    #[tokio::test]
    async fn error_api_wiring() {
        let leapsome_api = LeapsomeApi { client: MockClient };

        let response = leapsome_api.all_feedback(None).await;
        assert!(response.is_err());
    }
}
