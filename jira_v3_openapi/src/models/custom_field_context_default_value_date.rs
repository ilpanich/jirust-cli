/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldContextDefaultValueDate : The default value for a Date custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueDate {
    /// The default date in ISO format. Ignored if `useCurrent` is true.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    /// Whether to use the current date.
    #[serde(rename = "useCurrent", skip_serializing_if = "Option::is_none")]
    pub use_current: Option<bool>,
}

impl CustomFieldContextDefaultValueDate {
    /// The default value for a Date custom field.
    pub fn new(r#type: String) -> CustomFieldContextDefaultValueDate {
        CustomFieldContextDefaultValueDate {
            date: None,
            r#type,
            use_current: None,
        }
    }
}

