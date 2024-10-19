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

/// CustomFieldContextDefaultValueDateTime : The default value for a date time custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueDateTime {
    /// The default date-time in ISO format. Ignored if `useCurrent` is true.
    #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    /// Whether to use the current date.
    #[serde(rename = "useCurrent", skip_serializing_if = "Option::is_none")]
    pub use_current: Option<bool>,
}

impl CustomFieldContextDefaultValueDateTime {
    /// The default value for a date time custom field.
    pub fn new(r#type: String) -> CustomFieldContextDefaultValueDateTime {
        CustomFieldContextDefaultValueDateTime {
            date_time: None,
            r#type,
            use_current: None,
        }
    }
}

