/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OrderOfIssueTypes : An ordered list of issue type IDs and information about where to move them.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderOfIssueTypes {
    /// The ID of the issue type to place the moved issue types after. Required if `position` isn't provided.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A list of the issue type IDs to move. The order of the issue type IDs in the list is the order they are given after the move.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
    /// The position the issue types should be moved to. Required if `after` isn't provided.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

impl OrderOfIssueTypes {
    /// An ordered list of issue type IDs and information about where to move them.
    pub fn new(issue_type_ids: Vec<String>) -> OrderOfIssueTypes {
        OrderOfIssueTypes {
            after: None,
            issue_type_ids,
            position: None,
        }
    }
}
/// The position the issue types should be moved to. Required if `after` isn't provided.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Position {
    #[serde(rename = "First")]
    First,
    #[serde(rename = "Last")]
    Last,
}

impl Default for Position {
    fn default() -> Position {
        Self::First
    }
}

