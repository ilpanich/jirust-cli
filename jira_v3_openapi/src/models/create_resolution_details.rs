/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CreateResolutionDetails : Details of an issue resolution.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateResolutionDetails {
    /// The description of the resolution.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the resolution. Must be unique (case-insensitive).
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateResolutionDetails {
    /// Details of an issue resolution.
    pub fn new(name: String) -> CreateResolutionDetails {
        CreateResolutionDetails {
            description: None,
            name,
        }
    }
}

