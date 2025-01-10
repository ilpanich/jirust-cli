/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldContextDefaultValueForgeMultiUserField : Defaults for a Forge collection of users custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeMultiUserField {
    /// The IDs of the default users.
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueForgeMultiUserField {
    /// Defaults for a Forge collection of users custom field.
    pub fn new(account_ids: Vec<String>, context_id: String, r#type: String) -> CustomFieldContextDefaultValueForgeMultiUserField {
        CustomFieldContextDefaultValueForgeMultiUserField {
            account_ids,
            context_id,
            r#type,
        }
    }
}

