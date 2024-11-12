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

/// CustomFieldOptionCreate : Details of a custom field option to create.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldOptionCreate {
    /// Whether the option is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// For cascading options, the ID of the custom field object containing the cascading option.
    #[serde(rename = "optionId", skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    /// The value of the custom field option.
    #[serde(rename = "value")]
    pub value: String,
}

impl CustomFieldOptionCreate {
    /// Details of a custom field option to create.
    pub fn new(value: String) -> CustomFieldOptionCreate {
        CustomFieldOptionCreate {
            disabled: None,
            option_id: None,
            value,
        }
    }
}

