/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldContextDefaultValueFloat : Default value for a float (number) custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueFloat {
    /// The default floating-point number.
    #[serde(rename = "number")]
    pub number: f64,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueFloat {
    /// Default value for a float (number) custom field.
    pub fn new(number: f64, r#type: String) -> CustomFieldContextDefaultValueFloat {
        CustomFieldContextDefaultValueFloat {
            number,
            r#type,
        }
    }
}

