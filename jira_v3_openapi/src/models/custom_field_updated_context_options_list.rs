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

/// CustomFieldUpdatedContextOptionsList : A list of custom field options for a context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldUpdatedContextOptionsList {
    /// The updated custom field options.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<models::CustomFieldOptionUpdate>>,
}

impl CustomFieldUpdatedContextOptionsList {
    /// A list of custom field options for a context.
    pub fn new() -> CustomFieldUpdatedContextOptionsList {
        CustomFieldUpdatedContextOptionsList {
            options: None,
        }
    }
}

