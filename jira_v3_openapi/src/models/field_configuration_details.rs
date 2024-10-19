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

/// FieldConfigurationDetails : Details of a field configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfigurationDetails {
    /// The description of the field configuration.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the field configuration. Must be unique.
    #[serde(rename = "name")]
    pub name: String,
}

impl FieldConfigurationDetails {
    /// Details of a field configuration.
    pub fn new(name: String) -> FieldConfigurationDetails {
        FieldConfigurationDetails {
            description: None,
            name,
        }
    }
}

