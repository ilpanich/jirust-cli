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

/// WorkflowSchemeAssociation : The explicit association between issue types and a workflow in a workflow scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeAssociation {
    /// The issue types assigned to the workflow.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
    /// The ID of the workflow.
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

impl WorkflowSchemeAssociation {
    /// The explicit association between issue types and a workflow in a workflow scheme.
    pub fn new(issue_type_ids: Vec<String>, workflow_id: String) -> WorkflowSchemeAssociation {
        WorkflowSchemeAssociation {
            issue_type_ids,
            workflow_id,
        }
    }
}

