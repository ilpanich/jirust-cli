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

/// CustomFieldCreatedContextOptionsList : A list of custom field options for a context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldCreatedContextOptionsList {
    /// The created custom field options.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<models::CustomFieldContextOption>>,
}

impl CustomFieldCreatedContextOptionsList {
    /// A list of custom field options for a context.
    pub fn new() -> CustomFieldCreatedContextOptionsList {
        CustomFieldCreatedContextOptionsList {
            options: None,
        }
    }
}

