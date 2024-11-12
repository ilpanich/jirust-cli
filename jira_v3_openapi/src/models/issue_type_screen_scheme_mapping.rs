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

/// IssueTypeScreenSchemeMapping : The IDs of the screen schemes for the issue type IDs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeMapping {
    /// The ID of the issue type or *default*. Only issue types used in classic projects are accepted. An entry for *default* must be provided and defines the mapping for all issue types without a screen scheme.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the screen scheme. Only screen schemes used in classic projects are accepted.
    #[serde(rename = "screenSchemeId")]
    pub screen_scheme_id: String,
}

impl IssueTypeScreenSchemeMapping {
    /// The IDs of the screen schemes for the issue type IDs.
    pub fn new(issue_type_id: String, screen_scheme_id: String) -> IssueTypeScreenSchemeMapping {
        IssueTypeScreenSchemeMapping {
            issue_type_id,
            screen_scheme_id,
        }
    }
}

