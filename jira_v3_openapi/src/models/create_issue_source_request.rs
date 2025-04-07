/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIssueSourceRequest {
    /// The issue source type. This must be \"Board\", \"Project\" or \"Filter\".
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The issue source value. This must be a board ID if the type is \"Board\", a project ID if the type is \"Project\" or a filter ID if the type is \"Filter\".
    #[serde(rename = "value")]
    pub value: i64,
}

impl CreateIssueSourceRequest {
    pub fn new(r#type: Type, value: i64) -> CreateIssueSourceRequest {
        CreateIssueSourceRequest {
            r#type,
            value,
        }
    }
}
/// The issue source type. This must be \"Board\", \"Project\" or \"Filter\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Board")]
    Board,
    #[serde(rename = "Project")]
    Project,
    #[serde(rename = "Filter")]
    Filter,
}

impl Default for Type {
    fn default() -> Type {
        Self::Board
    }
}

