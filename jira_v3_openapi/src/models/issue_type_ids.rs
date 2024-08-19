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

/// IssueTypeIds : The list of issue type IDs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeIds {
    /// The list of issue type IDs.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
}

impl IssueTypeIds {
    /// The list of issue type IDs.
    pub fn new(issue_type_ids: Vec<String>) -> IssueTypeIds {
        IssueTypeIds {
            issue_type_ids,
        }
    }
}

