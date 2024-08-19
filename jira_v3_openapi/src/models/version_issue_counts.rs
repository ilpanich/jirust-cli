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

/// VersionIssueCounts : Various counts of issues within a version.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionIssueCounts {
    /// List of custom fields using the version.
    #[serde(rename = "customFieldUsage", skip_serializing_if = "Option::is_none")]
    pub custom_field_usage: Option<Vec<models::VersionUsageInCustomField>>,
    /// Count of issues where a version custom field is set to the version.
    #[serde(rename = "issueCountWithCustomFieldsShowingVersion", skip_serializing_if = "Option::is_none")]
    pub issue_count_with_custom_fields_showing_version: Option<i64>,
    /// Count of issues where the `affectedVersion` is set to the version.
    #[serde(rename = "issuesAffectedCount", skip_serializing_if = "Option::is_none")]
    pub issues_affected_count: Option<i64>,
    /// Count of issues where the `fixVersion` is set to the version.
    #[serde(rename = "issuesFixedCount", skip_serializing_if = "Option::is_none")]
    pub issues_fixed_count: Option<i64>,
    /// The URL of these count details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl VersionIssueCounts {
    /// Various counts of issues within a version.
    pub fn new() -> VersionIssueCounts {
        VersionIssueCounts {
            custom_field_usage: None,
            issue_count_with_custom_fields_showing_version: None,
            issues_affected_count: None,
            issues_fixed_count: None,
            param_self: None,
        }
    }
}

