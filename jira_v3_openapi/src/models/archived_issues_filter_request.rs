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

/// ArchivedIssuesFilterRequest : Details of a filter for exporting archived issues.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArchivedIssuesFilterRequest {
    /// List archived issues archived by a specified account ID.
    #[serde(rename = "archivedBy", skip_serializing_if = "Option::is_none")]
    pub archived_by: Option<Vec<String>>,
    #[serde(rename = "archivedDateRange", skip_serializing_if = "Option::is_none")]
    pub archived_date_range: Option<Box<models::DateRangeFilterRequest>>,
    /// List archived issues with a specified issue type ID.
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    /// List archived issues with a specified project key.
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    /// List archived issues where the reporter is a specified account ID.
    #[serde(rename = "reporters", skip_serializing_if = "Option::is_none")]
    pub reporters: Option<Vec<String>>,
}

impl ArchivedIssuesFilterRequest {
    /// Details of a filter for exporting archived issues.
    pub fn new() -> ArchivedIssuesFilterRequest {
        ArchivedIssuesFilterRequest {
            archived_by: None,
            archived_date_range: None,
            issue_types: None,
            projects: None,
            reporters: None,
        }
    }
}

