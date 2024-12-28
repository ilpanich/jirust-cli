/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAtlassianTeamResponse {
    /// The capacity for the Atlassian team.
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<f64>,
    /// The Atlassian team ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The ID of the issue source for the Atlassian team.
    #[serde(rename = "issueSourceId", skip_serializing_if = "Option::is_none")]
    pub issue_source_id: Option<i64>,
    /// The planning style for the Atlassian team. This is \"Scrum\" or \"Kanban\".
    #[serde(rename = "planningStyle")]
    pub planning_style: PlanningStyle,
    /// The sprint length for the Atlassian team.
    #[serde(rename = "sprintLength", skip_serializing_if = "Option::is_none")]
    pub sprint_length: Option<i64>,
}

impl GetAtlassianTeamResponse {
    pub fn new(id: String, planning_style: PlanningStyle) -> GetAtlassianTeamResponse {
        GetAtlassianTeamResponse {
            capacity: None,
            id,
            issue_source_id: None,
            planning_style,
            sprint_length: None,
        }
    }
}
/// The planning style for the Atlassian team. This is \"Scrum\" or \"Kanban\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlanningStyle {
    #[serde(rename = "Scrum")]
    Scrum,
    #[serde(rename = "Kanban")]
    Kanban,
}

impl Default for PlanningStyle {
    fn default() -> PlanningStyle {
        Self::Scrum
    }
}

