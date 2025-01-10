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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueLimitReportRequest {
    /// A list of fields and their respective approaching limit threshold. Required for querying issues approaching limits. Optional for querying issues breaching limits. Accepted fields are: `comment`, `worklog`, `attachment`, `remoteIssueLinks`, and `issuelinks`. Example: `{\"issuesApproachingLimitParams\": {\"comment\": 4500, \"attachment\": 1800}}`
    #[serde(rename = "issuesApproachingLimitParams", skip_serializing_if = "Option::is_none")]
    pub issues_approaching_limit_params: Option<std::collections::HashMap<String, i32>>,
}

impl IssueLimitReportRequest {
    pub fn new() -> IssueLimitReportRequest {
        IssueLimitReportRequest {
            issues_approaching_limit_params: None,
        }
    }
}

