/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TransitionScreenDetails : The details of a transition screen.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransitionScreenDetails {
    /// The ID of the screen.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the screen.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl TransitionScreenDetails {
    /// The details of a transition screen.
    pub fn new(id: String) -> TransitionScreenDetails {
        TransitionScreenDetails {
            id,
            name: None,
        }
    }
}

