/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowUpdateResponse {
    /// List of updated statuses.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<models::JiraWorkflowStatus>>,
    /// If there is a [asynchronous task](#async-operations) operation, as a result of this update.
    #[serde(rename = "taskId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<Option<String>>,
    /// List of updated workflows.
    #[serde(rename = "workflows", skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<models::JiraWorkflow>>,
}

impl WorkflowUpdateResponse {
    pub fn new() -> WorkflowUpdateResponse {
        WorkflowUpdateResponse {
            statuses: None,
            task_id: None,
            workflows: None,
        }
    }
}

