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
pub struct GetExclusionRulesResponse {
    /// The IDs of the issues excluded from the plan.
    #[serde(rename = "issueIds", skip_serializing_if = "Option::is_none")]
    pub issue_ids: Option<Vec<i64>>,
    /// The IDs of the issue types excluded from the plan.
    #[serde(rename = "issueTypeIds", skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<i64>>,
    /// Issues completed this number of days ago are excluded from the plan.
    #[serde(rename = "numberOfDaysToShowCompletedIssues")]
    pub number_of_days_to_show_completed_issues: i32,
    /// The IDs of the releases excluded from the plan.
    #[serde(rename = "releaseIds", skip_serializing_if = "Option::is_none")]
    pub release_ids: Option<Vec<i64>>,
    /// The IDs of the work status categories excluded from the plan.
    #[serde(rename = "workStatusCategoryIds", skip_serializing_if = "Option::is_none")]
    pub work_status_category_ids: Option<Vec<i64>>,
    /// The IDs of the work statuses excluded from the plan.
    #[serde(rename = "workStatusIds", skip_serializing_if = "Option::is_none")]
    pub work_status_ids: Option<Vec<i64>>,
}

impl GetExclusionRulesResponse {
    pub fn new(number_of_days_to_show_completed_issues: i32) -> GetExclusionRulesResponse {
        GetExclusionRulesResponse {
            issue_ids: None,
            issue_type_ids: None,
            number_of_days_to_show_completed_issues,
            release_ids: None,
            work_status_category_ids: None,
            work_status_ids: None,
        }
    }
}

