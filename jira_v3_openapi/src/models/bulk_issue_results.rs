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

/// BulkIssueResults : The list of requested issues & fields.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkIssueResults {
    /// When Jira can't return an issue enumerated in a request due to a retriable error or payload constraint, we'll return the respective issue ID with a corresponding error message. This list is empty when there are no errors Issues which aren't found or that the user doesn't have permission to view won't be returned in this list.
    #[serde(rename = "issueErrors", skip_serializing_if = "Option::is_none")]
    pub issue_errors: Option<Vec<models::IssueError>>,
    /// The list of issues.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<models::IssueBean>>,
}

impl BulkIssueResults {
    /// The list of requested issues & fields.
    pub fn new() -> BulkIssueResults {
        BulkIssueResults {
            issue_errors: None,
            issues: None,
        }
    }
}

