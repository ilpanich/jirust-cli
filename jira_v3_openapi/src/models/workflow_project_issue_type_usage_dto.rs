/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowProjectIssueTypeUsageDto : Issue types associated with the workflow for a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowProjectIssueTypeUsageDto {
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Box<models::WorkflowProjectIssueTypeUsagePage>>,
    /// The ID of the project.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The ID of the workflow.
    #[serde(rename = "workflowId", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl WorkflowProjectIssueTypeUsageDto {
    /// Issue types associated with the workflow for a project.
    pub fn new() -> WorkflowProjectIssueTypeUsageDto {
        WorkflowProjectIssueTypeUsageDto {
            issue_types: None,
            project_id: None,
            workflow_id: None,
        }
    }
}

