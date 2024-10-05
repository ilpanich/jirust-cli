/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CreatedIssues : Details about the issues created and the errors for requests that failed.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatedIssues {
    /// Error details for failed issue creation requests.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::BulkOperationErrorResult>>,
    /// Details of the issues created.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<models::CreatedIssue>>,
}

impl CreatedIssues {
    /// Details about the issues created and the errors for requests that failed.
    pub fn new() -> CreatedIssues {
        CreatedIssues {
            errors: None,
            issues: None,
        }
    }
}

