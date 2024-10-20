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

/// ConnectCustomFieldValues : Details of updates for a custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectCustomFieldValues {
    /// The list of custom field update details.
    #[serde(rename = "updateValueList", skip_serializing_if = "Option::is_none")]
    pub update_value_list: Option<Vec<models::ConnectCustomFieldValue>>,
}

impl ConnectCustomFieldValues {
    /// Details of updates for a custom field.
    pub fn new() -> ConnectCustomFieldValues {
        ConnectCustomFieldValues {
            update_value_list: None,
        }
    }
}

