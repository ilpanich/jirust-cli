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

/// JqlQueriesToSanitize : The list of JQL queries to sanitize for the given account IDs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueriesToSanitize {
    /// The list of JQL queries to sanitize. Must contain unique values. Maximum of 20 queries.
    #[serde(rename = "queries")]
    pub queries: Vec<models::JqlQueryToSanitize>,
}

impl JqlQueriesToSanitize {
    /// The list of JQL queries to sanitize for the given account IDs.
    pub fn new(queries: Vec<models::JqlQueryToSanitize>) -> JqlQueriesToSanitize {
        JqlQueriesToSanitize {
            queries,
        }
    }
}

