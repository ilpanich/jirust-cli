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

/// IssueTypeScreenSchemeItem : The screen scheme for an issue type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeItem {
    /// The ID of the issue type or *default*. Only issue types used in classic projects are accepted. When creating an issue screen scheme, an entry for *default* must be provided and defines the mapping for all issue types without a screen scheme. Otherwise, a *default* entry can't be provided.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the issue type screen scheme.
    #[serde(rename = "issueTypeScreenSchemeId")]
    pub issue_type_screen_scheme_id: String,
    /// The ID of the screen scheme.
    #[serde(rename = "screenSchemeId")]
    pub screen_scheme_id: String,
}

impl IssueTypeScreenSchemeItem {
    /// The screen scheme for an issue type.
    pub fn new(issue_type_id: String, issue_type_screen_scheme_id: String, screen_scheme_id: String) -> IssueTypeScreenSchemeItem {
        IssueTypeScreenSchemeItem {
            issue_type_id,
            issue_type_screen_scheme_id,
            screen_scheme_id,
        }
    }
}

