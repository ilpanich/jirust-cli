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

/// BulkContextualConfiguration : Details of the contextual configuration for a custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkContextualConfiguration {
    /// The field configuration.
    #[serde(rename = "configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Option<serde_json::Value>>,
    /// The ID of the custom field.
    #[serde(rename = "customFieldId")]
    pub custom_field_id: String,
    /// The ID of the field context the configuration is associated with.
    #[serde(rename = "fieldContextId")]
    pub field_context_id: String,
    /// The ID of the configuration.
    #[serde(rename = "id")]
    pub id: String,
    /// The field value schema.
    #[serde(rename = "schema", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Option<serde_json::Value>>,
}

impl BulkContextualConfiguration {
    /// Details of the contextual configuration for a custom field.
    pub fn new(custom_field_id: String, field_context_id: String, id: String) -> BulkContextualConfiguration {
        BulkContextualConfiguration {
            configuration: None,
            custom_field_id,
            field_context_id,
            id,
            schema: None,
        }
    }
}

