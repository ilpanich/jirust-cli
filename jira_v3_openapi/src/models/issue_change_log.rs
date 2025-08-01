/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueChangeLog : List of changelogs that belong to single issue
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueChangeLog {
    /// List of changelogs that belongs to given issueId.
    #[serde(rename = "changeHistories", skip_serializing_if = "Option::is_none")]
    pub change_histories: Option<Vec<models::Changelog>>,
    /// The ID of the issue.
    #[serde(rename = "issueId", skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
}

impl IssueChangeLog {
    /// List of changelogs that belong to single issue
    pub fn new() -> IssueChangeLog {
        IssueChangeLog {
            change_histories: None,
            issue_id: None,
        }
    }
}

