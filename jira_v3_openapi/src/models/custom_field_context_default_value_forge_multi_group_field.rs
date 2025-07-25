/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldContextDefaultValueForgeMultiGroupField : The default value for a Forge collection of groups custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeMultiGroupField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The IDs of the default groups.
    #[serde(rename = "groupIds")]
    pub group_ids: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueForgeMultiGroupField {
    /// The default value for a Forge collection of groups custom field.
    pub fn new(context_id: String, group_ids: Vec<String>, r#type: String) -> CustomFieldContextDefaultValueForgeMultiGroupField {
        CustomFieldContextDefaultValueForgeMultiGroupField {
            context_id,
            group_ids,
            r#type,
        }
    }
}

