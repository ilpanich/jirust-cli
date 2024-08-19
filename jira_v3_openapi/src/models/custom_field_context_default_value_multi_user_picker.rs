/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldContextDefaultValueMultiUserPicker : The default value for a User Picker (multiple) custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueMultiUserPicker {
    /// The IDs of the default users.
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueMultiUserPicker {
    /// The default value for a User Picker (multiple) custom field.
    pub fn new(account_ids: Vec<String>, context_id: String, r#type: String) -> CustomFieldContextDefaultValueMultiUserPicker {
        CustomFieldContextDefaultValueMultiUserPicker {
            account_ids,
            context_id,
            r#type,
        }
    }
}

