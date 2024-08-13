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

/// FieldConfigurationItem : A field within a field configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfigurationItem {
    /// The description of the field within the field configuration.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the field within the field configuration.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the field is hidden in the field configuration.
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    /// Whether the field is required in the field configuration.
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// The renderer type for the field within the field configuration.
    #[serde(rename = "renderer", skip_serializing_if = "Option::is_none")]
    pub renderer: Option<String>,
}

impl FieldConfigurationItem {
    /// A field within a field configuration.
    pub fn new(id: String) -> FieldConfigurationItem {
        FieldConfigurationItem {
            description: None,
            id,
            is_hidden: None,
            is_required: None,
            renderer: None,
        }
    }
}
