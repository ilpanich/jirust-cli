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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedResponse {
    #[serde(rename = "errorCollection", skip_serializing_if = "Option::is_none")]
    pub error_collection: Option<Box<models::ErrorCollection>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "warningCollection", skip_serializing_if = "Option::is_none")]
    pub warning_collection: Option<Box<models::WarningCollection>>,
}

impl NestedResponse {
    pub fn new() -> NestedResponse {
        NestedResponse {
            error_collection: None,
            status: None,
            warning_collection: None,
        }
    }
}

