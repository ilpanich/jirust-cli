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

/// CustomFieldContextDefaultValueMultipleGroupPicker : The default value for a multiple group picker custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueMultipleGroupPicker {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The IDs of the default groups.
    #[serde(rename = "groupIds")]
    pub group_ids: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueMultipleGroupPicker {
    /// The default value for a multiple group picker custom field.
    pub fn new(context_id: String, group_ids: Vec<String>, r#type: String) -> CustomFieldContextDefaultValueMultipleGroupPicker {
        CustomFieldContextDefaultValueMultipleGroupPicker {
            context_id,
            group_ids,
            r#type,
        }
    }
}

