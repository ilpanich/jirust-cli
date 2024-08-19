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

/// ProjectIssueTypes : Use the optional `workflows.usages` expand to get additional information about the projects and issue types associated with the requested workflows.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectIssueTypes {
    /// IDs of the issue types
    #[serde(rename = "issueTypes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Option<Vec<String>>>,
    #[serde(rename = "project", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project: Option<Option<Box<models::ProjectId>>>,
}

impl ProjectIssueTypes {
    /// Use the optional `workflows.usages` expand to get additional information about the projects and issue types associated with the requested workflows.
    pub fn new() -> ProjectIssueTypes {
        ProjectIssueTypes {
            issue_types: None,
            project: None,
        }
    }
}

