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

/// FieldConfigurationScheme : Details of a field configuration scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfigurationScheme {
    /// The description of the field configuration scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the field configuration scheme.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the field configuration scheme.
    #[serde(rename = "name")]
    pub name: String,
}

impl FieldConfigurationScheme {
    /// Details of a field configuration scheme.
    pub fn new(id: String, name: String) -> FieldConfigurationScheme {
        FieldConfigurationScheme {
            description: None,
            id,
            name,
        }
    }
}

