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

/// CustomFieldContextDefaultValueSingleVersionPicker : The default value for a version picker custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueSingleVersionPicker {
    #[serde(rename = "type")]
    pub r#type: String,
    /// The ID of the default version.
    #[serde(rename = "versionId")]
    pub version_id: String,
    /// The order the pickable versions are displayed in. If not provided, the released-first order is used. Available version orders are `\"releasedFirst\"` and `\"unreleasedFirst\"`.
    #[serde(rename = "versionOrder", skip_serializing_if = "Option::is_none")]
    pub version_order: Option<String>,
}

impl CustomFieldContextDefaultValueSingleVersionPicker {
    /// The default value for a version picker custom field.
    pub fn new(r#type: String, version_id: String) -> CustomFieldContextDefaultValueSingleVersionPicker {
        CustomFieldContextDefaultValueSingleVersionPicker {
            r#type,
            version_id,
            version_order: None,
        }
    }
}

