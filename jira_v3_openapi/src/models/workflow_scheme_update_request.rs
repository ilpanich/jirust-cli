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

/// WorkflowSchemeUpdateRequest : The update workflow scheme payload.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeUpdateRequest {
    /// The ID of the workflow for issue types without having a mapping defined in this workflow scheme. Only used in global-scoped workflow schemes. If the `defaultWorkflowId` isn't specified, this is set to *Jira Workflow (jira)*.
    #[serde(rename = "defaultWorkflowId", skip_serializing_if = "Option::is_none")]
    pub default_workflow_id: Option<String>,
    /// The new description for this workflow scheme.
    #[serde(rename = "description")]
    pub description: String,
    /// The ID of this workflow scheme.
    #[serde(rename = "id")]
    pub id: String,
    /// The new name for this workflow scheme.
    #[serde(rename = "name")]
    pub name: String,
    /// Overrides, for the selected issue types, any status mappings provided in `statusMappingsByWorkflows`. Status mappings are required when the new workflow for an issue type doesn't contain all statuses that the old workflow has. Status mappings can be provided by a combination of `statusMappingsByWorkflows` and `statusMappingsByIssueTypeOverride`.
    #[serde(rename = "statusMappingsByIssueTypeOverride", skip_serializing_if = "Option::is_none")]
    pub status_mappings_by_issue_type_override: Option<Vec<models::MappingsByIssueTypeOverride>>,
    /// The status mappings by workflows. Status mappings are required when the new workflow for an issue type doesn't contain all statuses that the old workflow has. Status mappings can be provided by a combination of `statusMappingsByWorkflows` and `statusMappingsByIssueTypeOverride`.
    #[serde(rename = "statusMappingsByWorkflows", skip_serializing_if = "Option::is_none")]
    pub status_mappings_by_workflows: Option<Vec<models::MappingsByWorkflow>>,
    #[serde(rename = "version")]
    pub version: Box<models::DocumentVersion>,
    /// Mappings from workflows to issue types.
    #[serde(rename = "workflowsForIssueTypes", skip_serializing_if = "Option::is_none")]
    pub workflows_for_issue_types: Option<Vec<models::WorkflowSchemeAssociation>>,
}

impl WorkflowSchemeUpdateRequest {
    /// The update workflow scheme payload.
    pub fn new(description: String, id: String, name: String, version: models::DocumentVersion) -> WorkflowSchemeUpdateRequest {
        WorkflowSchemeUpdateRequest {
            default_workflow_id: None,
            description,
            id,
            name,
            status_mappings_by_issue_type_override: None,
            status_mappings_by_workflows: None,
            version: Box::new(version),
            workflows_for_issue_types: None,
        }
    }
}

