/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowProjectIssueTypeUsage : The issue type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowProjectIssueTypeUsage {
    /// The ID of the issue type.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl WorkflowProjectIssueTypeUsage {
    /// The issue type.
    pub fn new() -> WorkflowProjectIssueTypeUsage {
        WorkflowProjectIssueTypeUsage {
            id: None,
        }
    }
}

