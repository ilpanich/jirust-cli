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

/// IssueTypeSchemeUpdateDetails : Details of the name, description, and default issue type for an issue type scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeSchemeUpdateDetails {
    /// The ID of the default issue type of the issue type scheme.
    #[serde(rename = "defaultIssueTypeId", skip_serializing_if = "Option::is_none")]
    pub default_issue_type_id: Option<String>,
    /// The description of the issue type scheme. The maximum length is 4000 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the issue type scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl IssueTypeSchemeUpdateDetails {
    /// Details of the name, description, and default issue type for an issue type scheme.
    pub fn new() -> IssueTypeSchemeUpdateDetails {
        IssueTypeSchemeUpdateDetails {
            default_issue_type_id: None,
            description: None,
            name: None,
        }
    }
}

