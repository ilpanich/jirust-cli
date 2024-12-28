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

/// IssueTypeWithStatus : Status details for an issue type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeWithStatus {
    /// The ID of the issue type.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the issue type.
    #[serde(rename = "name")]
    pub name: String,
    /// The URL of the issue type's status details.
    #[serde(rename = "self")]
    pub param_self: String,
    /// List of status details for the issue type.
    #[serde(rename = "statuses")]
    pub statuses: Vec<models::StatusDetails>,
    /// Whether this issue type represents subtasks.
    #[serde(rename = "subtask")]
    pub subtask: bool,
}

impl IssueTypeWithStatus {
    /// Status details for an issue type.
    pub fn new(id: String, name: String, param_self: String, statuses: Vec<models::StatusDetails>, subtask: bool) -> IssueTypeWithStatus {
        IssueTypeWithStatus {
            id,
            name,
            param_self,
            statuses,
            subtask,
        }
    }
}

