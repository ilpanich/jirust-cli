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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySchemeLevelMemberBean {
    /// The value corresponding to the specified member type.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// The issue security level member type, e.g `reporter`, `group`, `user`, `projectrole`, `applicationRole`.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl SecuritySchemeLevelMemberBean {
    pub fn new(r#type: String) -> SecuritySchemeLevelMemberBean {
        SecuritySchemeLevelMemberBean {
            parameter: None,
            r#type,
        }
    }
}

