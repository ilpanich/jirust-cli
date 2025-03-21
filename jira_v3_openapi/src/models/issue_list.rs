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

/// IssueList : A list of issue IDs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueList {
    /// The list of issue IDs.
    #[serde(rename = "issueIds")]
    pub issue_ids: Vec<String>,
}

impl IssueList {
    /// A list of issue IDs.
    pub fn new(issue_ids: Vec<String>) -> IssueList {
        IssueList {
            issue_ids,
        }
    }
}

