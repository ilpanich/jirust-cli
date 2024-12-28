/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SecuritySchemeId : The ID of the issue security scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySchemeId {
    /// The ID of the issue security scheme.
    #[serde(rename = "id")]
    pub id: String,
}

impl SecuritySchemeId {
    /// The ID of the issue security scheme.
    pub fn new(id: String) -> SecuritySchemeId {
        SecuritySchemeId {
            id,
        }
    }
}

