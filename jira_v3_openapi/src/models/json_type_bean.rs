/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JsonTypeBean : The schema of a field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonTypeBean {
    /// If the field is a custom field, the configuration of the field.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// If the field is a custom field, the URI of the field.
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    /// If the field is a custom field, the custom ID of the field.
    #[serde(rename = "customId", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<i64>,
    /// When the data type is an array, the name of the field items within the array.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    /// If the field is a system field, the name of the field.
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// The data type of the field.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl JsonTypeBean {
    /// The schema of a field.
    pub fn new(r#type: String) -> JsonTypeBean {
        JsonTypeBean {
            configuration: None,
            custom: None,
            custom_id: None,
            items: None,
            system: None,
            r#type,
        }
    }
}

