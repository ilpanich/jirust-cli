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

/// ResolutionId : The ID of an issue resolution.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolutionId {
    /// The ID of the issue resolution.
    #[serde(rename = "id")]
    pub id: String,
}

impl ResolutionId {
    /// The ID of an issue resolution.
    pub fn new(id: String) -> ResolutionId {
        ResolutionId {
            id,
        }
    }
}

