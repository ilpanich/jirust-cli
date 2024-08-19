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

/// FieldConfigurationItemsDetails : Details of field configuration items.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfigurationItemsDetails {
    /// Details of fields in a field configuration.
    #[serde(rename = "fieldConfigurationItems")]
    pub field_configuration_items: Vec<models::FieldConfigurationItem>,
}

impl FieldConfigurationItemsDetails {
    /// Details of field configuration items.
    pub fn new(field_configuration_items: Vec<models::FieldConfigurationItem>) -> FieldConfigurationItemsDetails {
        FieldConfigurationItemsDetails {
            field_configuration_items,
        }
    }
}

