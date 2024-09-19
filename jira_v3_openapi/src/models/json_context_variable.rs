/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JsonContextVariable : A JSON object with custom content.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonContextVariable {
    /// Type of custom context variable.
    #[serde(rename = "type")]
    pub r#type: String,
    /// A JSON object containing custom content.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl JsonContextVariable {
    /// A JSON object with custom content.
    pub fn new(r#type: String) -> JsonContextVariable {
        JsonContextVariable {
            r#type,
            value: None,
        }
    }
}

