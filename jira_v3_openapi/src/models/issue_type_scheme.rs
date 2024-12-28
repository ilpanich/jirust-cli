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

/// IssueTypeScheme : Details of an issue type scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScheme {
    /// The ID of the default issue type of the issue type scheme.
    #[serde(rename = "defaultIssueTypeId", skip_serializing_if = "Option::is_none")]
    pub default_issue_type_id: Option<String>,
    /// The description of the issue type scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue type scheme.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the issue type scheme is the default.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The name of the issue type scheme.
    #[serde(rename = "name")]
    pub name: String,
}

impl IssueTypeScheme {
    /// Details of an issue type scheme.
    pub fn new(id: String, name: String) -> IssueTypeScheme {
        IssueTypeScheme {
            default_issue_type_id: None,
            description: None,
            id,
            is_default: None,
            name,
        }
    }
}

