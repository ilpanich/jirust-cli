/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FieldConfiguration : Details of a field configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfiguration {
    /// The description of the field configuration.
    #[serde(rename = "description")]
    pub description: String,
    /// The ID of the field configuration.
    #[serde(rename = "id")]
    pub id: i64,
    /// Whether the field configuration is the default.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The name of the field configuration.
    #[serde(rename = "name")]
    pub name: String,
}

impl FieldConfiguration {
    /// Details of a field configuration.
    pub fn new(description: String, id: i64, name: String) -> FieldConfiguration {
        FieldConfiguration {
            description,
            id,
            is_default: None,
            name,
        }
    }
}

