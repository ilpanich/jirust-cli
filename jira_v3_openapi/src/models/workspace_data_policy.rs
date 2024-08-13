/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkspaceDataPolicy : Details about data policy.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceDataPolicy {
    /// Whether the workspace contains any content inaccessible to the requesting application.
    #[serde(rename = "anyContentBlocked", skip_serializing_if = "Option::is_none")]
    pub any_content_blocked: Option<bool>,
}

impl WorkspaceDataPolicy {
    /// Details about data policy.
    pub fn new() -> WorkspaceDataPolicy {
        WorkspaceDataPolicy {
            any_content_blocked: None,
        }
    }
}
