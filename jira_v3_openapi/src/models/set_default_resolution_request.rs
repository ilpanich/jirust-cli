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

/// SetDefaultResolutionRequest : The new default issue resolution.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetDefaultResolutionRequest {
    /// The ID of the new default issue resolution. Must be an existing ID or null. Setting this to null erases the default resolution setting.
    #[serde(rename = "id")]
    pub id: String,
}

impl SetDefaultResolutionRequest {
    /// The new default issue resolution.
    pub fn new(id: String) -> SetDefaultResolutionRequest {
        SetDefaultResolutionRequest {
            id,
        }
    }
}

