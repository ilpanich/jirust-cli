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

/// IssueTypeIdsToRemove : The list of issue type IDs to be removed from the field configuration scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeIdsToRemove {
    /// The list of issue type IDs. Must contain unique values not longer than 255 characters and not be empty. Maximum of 100 IDs.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
}

impl IssueTypeIdsToRemove {
    /// The list of issue type IDs to be removed from the field configuration scheme.
    pub fn new(issue_type_ids: Vec<String>) -> IssueTypeIdsToRemove {
        IssueTypeIdsToRemove {
            issue_type_ids,
        }
    }
}

