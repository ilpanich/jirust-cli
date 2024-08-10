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

/// ErrorCollection : Error messages from an operation.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorCollection {
    /// The list of error messages produced by this operation. For example, \"input parameter 'key' must be provided\"
    #[serde(rename = "errorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<String>>,
    /// The list of errors by parameter returned by the operation. For example,\"projectKey\": \"Project keys must start with an uppercase letter, followed by one or more uppercase alphanumeric characters.\"
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

impl ErrorCollection {
    /// Error messages from an operation.
    pub fn new() -> ErrorCollection {
        ErrorCollection {
            error_messages: None,
            errors: None,
            status: None,
        }
    }
}

