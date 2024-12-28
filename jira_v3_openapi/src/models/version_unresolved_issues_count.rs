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

/// VersionUnresolvedIssuesCount : Count of a version's unresolved issues.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionUnresolvedIssuesCount {
    /// Count of issues.
    #[serde(rename = "issuesCount", skip_serializing_if = "Option::is_none")]
    pub issues_count: Option<i64>,
    /// Count of unresolved issues.
    #[serde(rename = "issuesUnresolvedCount", skip_serializing_if = "Option::is_none")]
    pub issues_unresolved_count: Option<i64>,
    /// The URL of these count details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl VersionUnresolvedIssuesCount {
    /// Count of a version's unresolved issues.
    pub fn new() -> VersionUnresolvedIssuesCount {
        VersionUnresolvedIssuesCount {
            issues_count: None,
            issues_unresolved_count: None,
            param_self: None,
        }
    }
}

