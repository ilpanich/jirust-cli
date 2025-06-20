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

/// CreateWorkflowStatusDetails : The details of a transition status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWorkflowStatusDetails {
    /// The ID of the status.
    #[serde(rename = "id")]
    pub id: String,
    /// The properties of the status.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

impl CreateWorkflowStatusDetails {
    /// The details of a transition status.
    pub fn new(id: String) -> CreateWorkflowStatusDetails {
        CreateWorkflowStatusDetails {
            id,
            properties: None,
        }
    }
}

