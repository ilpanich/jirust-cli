/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueTypesWorkflowMapping : Details about the mapping between issue types and a workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypesWorkflowMapping {
    /// Whether the workflow is the default workflow for the workflow scheme.
    #[serde(rename = "defaultMapping", skip_serializing_if = "Option::is_none")]
    pub default_mapping: Option<bool>,
    /// The list of issue type IDs.
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    /// Whether a draft workflow scheme is created or updated when updating an active workflow scheme. The draft is updated with the new workflow-issue types mapping. Defaults to `false`.
    #[serde(rename = "updateDraftIfNeeded", skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    /// The name of the workflow. Optional if updating the workflow-issue types mapping.
    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}

impl IssueTypesWorkflowMapping {
    /// Details about the mapping between issue types and a workflow.
    pub fn new() -> IssueTypesWorkflowMapping {
        IssueTypesWorkflowMapping {
            default_mapping: None,
            issue_types: None,
            update_draft_if_needed: None,
            workflow: None,
        }
    }
}

