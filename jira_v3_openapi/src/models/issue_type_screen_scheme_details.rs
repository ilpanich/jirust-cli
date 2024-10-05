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

/// IssueTypeScreenSchemeDetails : The details of an issue type screen scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeDetails {
    /// The description of the issue type screen scheme. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The IDs of the screen schemes for the issue type IDs and *default*. A *default* entry is required to create an issue type screen scheme, it defines the mapping for all issue types without a screen scheme.
    #[serde(rename = "issueTypeMappings")]
    pub issue_type_mappings: Vec<models::IssueTypeScreenSchemeMapping>,
    /// The name of the issue type screen scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name")]
    pub name: String,
}

impl IssueTypeScreenSchemeDetails {
    /// The details of an issue type screen scheme.
    pub fn new(issue_type_mappings: Vec<models::IssueTypeScreenSchemeMapping>, name: String) -> IssueTypeScreenSchemeDetails {
        IssueTypeScreenSchemeDetails {
            description: None,
            issue_type_mappings,
            name,
        }
    }
}

