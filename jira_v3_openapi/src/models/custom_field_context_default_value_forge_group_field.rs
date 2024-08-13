/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldContextDefaultValueForgeGroupField : The default value for a Forge group custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeGroupField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the the default group.
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueForgeGroupField {
    /// The default value for a Forge group custom field.
    pub fn new(context_id: String, group_id: String, r#type: String) -> CustomFieldContextDefaultValueForgeGroupField {
        CustomFieldContextDefaultValueForgeGroupField {
            context_id,
            group_id,
            r#type,
        }
    }
}
