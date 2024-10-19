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

/// WorkflowTransition : A workflow transition.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransition {
    /// The transition ID.
    #[serde(rename = "id")]
    pub id: i32,
    /// The transition name.
    #[serde(rename = "name")]
    pub name: String,
}

impl WorkflowTransition {
    /// A workflow transition.
    pub fn new(id: i32, name: String) -> WorkflowTransition {
        WorkflowTransition {
            id,
            name,
        }
    }
}

