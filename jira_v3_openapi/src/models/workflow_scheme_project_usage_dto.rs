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

/// WorkflowSchemeProjectUsageDto : Projects using the workflow scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeProjectUsageDto {
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Box<models::ProjectUsagePage>>,
    /// The workflow scheme ID.
    #[serde(rename = "workflowSchemeId", skip_serializing_if = "Option::is_none")]
    pub workflow_scheme_id: Option<String>,
}

impl WorkflowSchemeProjectUsageDto {
    /// Projects using the workflow scheme.
    pub fn new() -> WorkflowSchemeProjectUsageDto {
        WorkflowSchemeProjectUsageDto {
            projects: None,
            workflow_scheme_id: None,
        }
    }
}

