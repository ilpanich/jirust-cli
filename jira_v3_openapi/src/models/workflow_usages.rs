/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowUsages : The workflows that use this status. Only available if the `workflowUsages` expand is requested.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowUsages {
    /// Workflow ID.
    #[serde(rename = "workflowId", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    /// Workflow name.
    #[serde(rename = "workflowName", skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

impl WorkflowUsages {
    /// The workflows that use this status. Only available if the `workflowUsages` expand is requested.
    pub fn new() -> WorkflowUsages {
        WorkflowUsages {
            workflow_id: None,
            workflow_name: None,
        }
    }
}

