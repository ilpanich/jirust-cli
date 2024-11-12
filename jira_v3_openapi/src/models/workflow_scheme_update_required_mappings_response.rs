/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeUpdateRequiredMappingsResponse {
    /// The list of required status mappings by issue type.
    #[serde(rename = "statusMappingsByIssueTypes", skip_serializing_if = "Option::is_none")]
    pub status_mappings_by_issue_types: Option<Vec<models::RequiredMappingByIssueType>>,
    /// The list of required status mappings by workflow.
    #[serde(rename = "statusMappingsByWorkflows", skip_serializing_if = "Option::is_none")]
    pub status_mappings_by_workflows: Option<Vec<models::RequiredMappingByWorkflows>>,
    /// The details of the statuses in the associated workflows.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<models::StatusMetadata>>,
    /// The statuses associated with each workflow.
    #[serde(rename = "statusesPerWorkflow", skip_serializing_if = "Option::is_none")]
    pub statuses_per_workflow: Option<Vec<models::StatusesPerWorkflow>>,
}

impl WorkflowSchemeUpdateRequiredMappingsResponse {
    pub fn new() -> WorkflowSchemeUpdateRequiredMappingsResponse {
        WorkflowSchemeUpdateRequiredMappingsResponse {
            status_mappings_by_issue_types: None,
            status_mappings_by_workflows: None,
            statuses: None,
            statuses_per_workflow: None,
        }
    }
}

