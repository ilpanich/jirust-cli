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
pub struct CreatePlanRequest {
    /// The cross-project releases to include in the plan.
    #[serde(rename = "crossProjectReleases", skip_serializing_if = "Option::is_none")]
    pub cross_project_releases: Option<Vec<models::CreateCrossProjectReleaseRequest>>,
    /// The custom fields for the plan.
    #[serde(rename = "customFields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<models::CreateCustomFieldRequest>>,
    /// The exclusion rules for the plan.
    #[serde(rename = "exclusionRules", skip_serializing_if = "Option::is_none")]
    pub exclusion_rules: Option<Box<models::CreateExclusionRulesRequest>>,
    /// The issue sources to include in the plan.
    #[serde(rename = "issueSources")]
    pub issue_sources: Vec<models::CreateIssueSourceRequest>,
    /// The account ID of the plan lead.
    #[serde(rename = "leadAccountId", skip_serializing_if = "Option::is_none")]
    pub lead_account_id: Option<String>,
    /// The plan name.
    #[serde(rename = "name")]
    pub name: String,
    /// The permissions for the plan.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<models::CreatePermissionRequest>>,
    /// The scheduling settings for the plan.
    #[serde(rename = "scheduling")]
    pub scheduling: Box<models::CreateSchedulingRequest>,
}

impl CreatePlanRequest {
    pub fn new(issue_sources: Vec<models::CreateIssueSourceRequest>, name: String, scheduling: models::CreateSchedulingRequest) -> CreatePlanRequest {
        CreatePlanRequest {
            cross_project_releases: None,
            custom_fields: None,
            exclusion_rules: None,
            issue_sources,
            lead_account_id: None,
            name,
            permissions: None,
            scheduling: Box::new(scheduling),
        }
    }
}

