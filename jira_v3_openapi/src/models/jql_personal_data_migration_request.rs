/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JqlPersonalDataMigrationRequest : The JQL queries to be converted.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlPersonalDataMigrationRequest {
    /// A list of queries with user identifiers. Maximum of 100 queries.
    #[serde(rename = "queryStrings", skip_serializing_if = "Option::is_none")]
    pub query_strings: Option<Vec<String>>,
}

impl JqlPersonalDataMigrationRequest {
    /// The JQL queries to be converted.
    pub fn new() -> JqlPersonalDataMigrationRequest {
        JqlPersonalDataMigrationRequest {
            query_strings: None,
        }
    }
}

