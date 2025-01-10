/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// VersionIssuesStatus : Counts of the number of issues in various statuses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionIssuesStatus {
    /// Count of issues with status *done*.
    #[serde(rename = "done", skip_serializing_if = "Option::is_none")]
    pub done: Option<i64>,
    /// Count of issues with status *in progress*.
    #[serde(rename = "inProgress", skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<i64>,
    /// Count of issues with status *to do*.
    #[serde(rename = "toDo", skip_serializing_if = "Option::is_none")]
    pub to_do: Option<i64>,
    /// Count of issues with a status other than *to do*, *in progress*, and *done*.
    #[serde(rename = "unmapped", skip_serializing_if = "Option::is_none")]
    pub unmapped: Option<i64>,
}

impl VersionIssuesStatus {
    /// Counts of the number of issues in various statuses.
    pub fn new() -> VersionIssuesStatus {
        VersionIssuesStatus {
            done: None,
            in_progress: None,
            to_do: None,
            unmapped: None,
        }
    }
}

