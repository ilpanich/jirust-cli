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

/// IssueArchivalSyncResponse : Number of archived/unarchived issues and list of errors that occurred during the action, if any.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueArchivalSyncResponse {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<models::Errors>>,
    #[serde(rename = "numberOfIssuesUpdated", skip_serializing_if = "Option::is_none")]
    pub number_of_issues_updated: Option<i64>,
}

impl IssueArchivalSyncResponse {
    /// Number of archived/unarchived issues and list of errors that occurred during the action, if any.
    pub fn new() -> IssueArchivalSyncResponse {
        IssueArchivalSyncResponse {
            errors: None,
            number_of_issues_updated: None,
        }
    }
}

