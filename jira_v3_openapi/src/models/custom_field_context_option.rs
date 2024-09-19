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

/// CustomFieldContextOption : Details of the custom field options for a context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextOption {
    /// Whether the option is disabled.
    #[serde(rename = "disabled")]
    pub disabled: bool,
    /// The ID of the custom field option.
    #[serde(rename = "id")]
    pub id: String,
    /// For cascading options, the ID of the custom field option containing the cascading option.
    #[serde(rename = "optionId", skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    /// The value of the custom field option.
    #[serde(rename = "value")]
    pub value: String,
}

impl CustomFieldContextOption {
    /// Details of the custom field options for a context.
    pub fn new(disabled: bool, id: String, value: String) -> CustomFieldContextOption {
        CustomFieldContextOption {
            disabled,
            id,
            option_id: None,
            value,
        }
    }
}

