/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateIssueSecurityLevelDetails : Details of issue security scheme level.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIssueSecurityLevelDetails {
    /// The description of the issue security scheme level.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the issue security scheme level. Must be unique.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateIssueSecurityLevelDetails {
    /// Details of issue security scheme level.
    pub fn new() -> UpdateIssueSecurityLevelDetails {
        UpdateIssueSecurityLevelDetails {
            description: None,
            name: None,
        }
    }
}

