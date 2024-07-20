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

/// UpdateDefaultScreenScheme : The ID of a screen scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDefaultScreenScheme {
    /// The ID of the screen scheme.
    #[serde(rename = "screenSchemeId")]
    pub screen_scheme_id: String,
}

impl UpdateDefaultScreenScheme {
    /// The ID of a screen scheme.
    pub fn new(screen_scheme_id: String) -> UpdateDefaultScreenScheme {
        UpdateDefaultScreenScheme {
            screen_scheme_id,
        }
    }
}

