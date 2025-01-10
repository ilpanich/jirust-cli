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

/// CustomFieldContextSingleUserPickerDefaults : Defaults for a User Picker (single) custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextSingleUserPickerDefaults {
    /// The ID of the default user.
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "userFilter")]
    pub user_filter: Box<models::UserFilter>,
}

impl CustomFieldContextSingleUserPickerDefaults {
    /// Defaults for a User Picker (single) custom field.
    pub fn new(account_id: String, context_id: String, r#type: String, user_filter: models::UserFilter) -> CustomFieldContextSingleUserPickerDefaults {
        CustomFieldContextSingleUserPickerDefaults {
            account_id,
            context_id,
            r#type,
            user_filter: Box::new(user_filter),
        }
    }
}

