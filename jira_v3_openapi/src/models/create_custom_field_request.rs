/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCustomFieldRequest {
    /// The custom field ID.
    #[serde(rename = "customFieldId")]
    pub custom_field_id: i64,
    /// Allows filtering issues based on their values for the custom field.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<bool>,
}

impl CreateCustomFieldRequest {
    pub fn new(custom_field_id: i64) -> CreateCustomFieldRequest {
        CreateCustomFieldRequest {
            custom_field_id,
            filter: None,
        }
    }
}
