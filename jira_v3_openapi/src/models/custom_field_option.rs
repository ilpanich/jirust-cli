/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldOption : Details of a custom option for a field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldOption {
    /// The URL of these custom field option details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The value of the custom field option.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CustomFieldOption {
    /// Details of a custom option for a field.
    pub fn new() -> CustomFieldOption {
        CustomFieldOption {
            param_self: None,
            value: None,
        }
    }
}

