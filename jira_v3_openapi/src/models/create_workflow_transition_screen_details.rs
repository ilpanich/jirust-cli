/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CreateWorkflowTransitionScreenDetails : The details of a transition screen.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWorkflowTransitionScreenDetails {
    /// The ID of the screen.
    #[serde(rename = "id")]
    pub id: String,
}

impl CreateWorkflowTransitionScreenDetails {
    /// The details of a transition screen.
    pub fn new(id: String) -> CreateWorkflowTransitionScreenDetails {
        CreateWorkflowTransitionScreenDetails {
            id,
        }
    }
}

