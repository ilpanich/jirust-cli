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

/// WorkflowReadResponse : Details of workflows and related statuses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowReadResponse {
    /// List of statuses.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<models::JiraWorkflowStatus>>,
    /// List of workflows.
    #[serde(rename = "workflows", skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<models::JiraWorkflow>>,
}

impl WorkflowReadResponse {
    /// Details of workflows and related statuses.
    pub fn new() -> WorkflowReadResponse {
        WorkflowReadResponse {
            statuses: None,
            workflows: None,
        }
    }
}

