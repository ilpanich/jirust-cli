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

/// DocumentVersion : The current version details of this workflow scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentVersion {
    /// The version UUID.
    #[serde(rename = "id")]
    pub id: String,
    /// The version number.
    #[serde(rename = "versionNumber")]
    pub version_number: i64,
}

impl DocumentVersion {
    /// The current version details of this workflow scheme.
    pub fn new(id: String, version_number: i64) -> DocumentVersion {
        DocumentVersion {
            id,
            version_number,
        }
    }
}

