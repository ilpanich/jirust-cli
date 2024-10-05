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

/// UpdateResolutionDetails : Details of an issue resolution.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateResolutionDetails {
    /// The description of the resolution.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the resolution. Must be unique.
    #[serde(rename = "name")]
    pub name: String,
}

impl UpdateResolutionDetails {
    /// Details of an issue resolution.
    pub fn new(name: String) -> UpdateResolutionDetails {
        UpdateResolutionDetails {
            description: None,
            name,
        }
    }
}

