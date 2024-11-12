/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldContextDefaultValueTextField : The default text for a text custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueTextField {
    /// The default text. The maximum length is 254 characters.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueTextField {
    /// The default text for a text custom field.
    pub fn new(r#type: String) -> CustomFieldContextDefaultValueTextField {
        CustomFieldContextDefaultValueTextField {
            text: None,
            r#type,
        }
    }
}

