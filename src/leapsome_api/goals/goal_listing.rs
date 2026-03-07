use std::collections::HashMap;

use crate::errors::AppErr;
use crate::leapsome_api::leapsome_req_res::LeapsomePost;

#[derive(Debug, serde::Serialize)]
struct GoalFilter {
    _id: String,
    name: String,
}

#[derive(Debug, serde::Serialize)]
struct GoalState {
    _id: String,
    name: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GoalCombinedFilter {
    goal_combined_filters: Vec<GoalFilter>,
    states: Vec<GoalState>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GoalsListRequest {
    list_custom_filter: GoalCombinedFilter,
    output: String,
    goal_ids_excluded_from_bulk: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalUser {
    displayed_name: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyResult {
    #[serde(rename = "_id")]
    id: Option<String>,
    name: Option<String>,
    progress: Option<f64>,
    metric_type: Option<String>,
    metric_current: Option<f64>,
    metric_end: Option<f64>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GoalTag {
    name: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalItem {
    #[serde(rename = "_id")]
    id: Option<String>,
    name: Option<String>,
    status: Option<String>,
    progress: Option<f64>,
    deadline: Option<String>,
    to_discuss: Option<String>,
    user: Option<GoalUser>,
    key_results: Option<Vec<KeyResult>>,
    tag_list: Option<Vec<GoalTag>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GoalsMapData {
    map: HashMap<String, Vec<GoalItem>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GoalsListResponse {
    data: GoalsMapData,
}

pub async fn list_goals<P: LeapsomePost>(client: &P) -> Result<GoalsListResponse, AppErr> {
    let req = GoalsListRequest {
        list_custom_filter: GoalCombinedFilter {
            goal_combined_filters: vec![GoalFilter {
                _id: "youAndTeam".to_string(),
                name: "You and your team(s)".to_string(),
            }],
            states: vec![
                GoalState {
                    _id: "live".to_string(),
                    name: "Active".to_string(),
                },
                GoalState {
                    _id: "draft".to_string(),
                    name: "Draft".to_string(),
                },
                GoalState {
                    _id: "approvalPending".to_string(),
                    name: "Approval pending".to_string(),
                },
            ],
        },
        output: "list".to_string(),
        goal_ids_excluded_from_bulk: vec![],
    };

    client.post(&req, "/goals/list").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialise_request() {
        let req = GoalsListRequest {
            list_custom_filter: GoalCombinedFilter {
                goal_combined_filters: vec![GoalFilter {
                    _id: "youAndTeam".to_string(),
                    name: "You and your team(s)".to_string(),
                }],
                states: vec![GoalState {
                    _id: "live".to_string(),
                    name: "Active".to_string(),
                }],
            },
            output: "list".to_string(),
            goal_ids_excluded_from_bulk: vec![],
        };
        let expected: serde_json::Value = serde_json::from_str(
            r#"
            {
                "listCustomFilter": {
                    "goalCombinedFilters": [
                        {"_id": "youAndTeam", "name": "You and your team(s)"}
                    ],
                    "states": [
                        {"_id": "live", "name": "Active"}
                    ]
                },
                "output": "list",
                "goalIdsExcludedFromBulk": []
            }
            "#,
        )
        .unwrap();
        let actual: serde_json::Value = serde_json::to_value(&req).unwrap();
        assert_eq!(actual, expected);
    }
}
