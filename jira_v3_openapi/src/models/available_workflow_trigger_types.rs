/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AvailableWorkflowTriggerTypes : The list of available trigger types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableWorkflowTriggerTypes {
    /// The description of the trigger rule.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the trigger rule.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type identifier of trigger rule.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl AvailableWorkflowTriggerTypes {
    /// The list of available trigger types.
    pub fn new() -> AvailableWorkflowTriggerTypes {
        AvailableWorkflowTriggerTypes {
            description: None,
            name: None,
            r#type: None,
        }
    }
}

