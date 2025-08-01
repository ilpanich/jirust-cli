/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowSchemeProjectAssociation : An associated workflow scheme and project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeProjectAssociation {
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// The ID of the workflow scheme. If the workflow scheme ID is `null`, the operation assigns the default workflow scheme.
    #[serde(rename = "workflowSchemeId", skip_serializing_if = "Option::is_none")]
    pub workflow_scheme_id: Option<String>,
}

impl WorkflowSchemeProjectAssociation {
    /// An associated workflow scheme and project.
    pub fn new(project_id: String) -> WorkflowSchemeProjectAssociation {
        WorkflowSchemeProjectAssociation {
            project_id,
            workflow_scheme_id: None,
        }
    }
}

