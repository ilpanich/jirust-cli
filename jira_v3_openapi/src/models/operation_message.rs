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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMessage {
    /// The human-readable message that describes the result.
    #[serde(rename = "message")]
    pub message: String,
    /// The status code of the response.
    #[serde(rename = "statusCode")]
    pub status_code: i32,
}

impl OperationMessage {
    pub fn new(message: String, status_code: i32) -> OperationMessage {
        OperationMessage {
            message,
            status_code,
        }
    }
}

