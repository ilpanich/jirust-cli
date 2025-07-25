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

/// StatusesPerWorkflow : The statuses associated with each workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusesPerWorkflow {
    /// The ID of the initial status for the workflow.
    #[serde(rename = "initialStatusId", skip_serializing_if = "Option::is_none")]
    pub initial_status_id: Option<String>,
    /// The status IDs associated with the workflow.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// The ID of the workflow.
    #[serde(rename = "workflowId", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl StatusesPerWorkflow {
    /// The statuses associated with each workflow.
    pub fn new() -> StatusesPerWorkflow {
        StatusesPerWorkflow {
            initial_status_id: None,
            statuses: None,
            workflow_id: None,
        }
    }
}

