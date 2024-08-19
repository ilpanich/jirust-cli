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

/// JqlQueryWithUnknownUsers : JQL queries that contained users that could not be found
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryWithUnknownUsers {
    /// The converted query, with accountIDs instead of user identifiers, or 'unknown' for users that could not be found
    #[serde(rename = "convertedQuery", skip_serializing_if = "Option::is_none")]
    pub converted_query: Option<String>,
    /// The original query, for reference
    #[serde(rename = "originalQuery", skip_serializing_if = "Option::is_none")]
    pub original_query: Option<String>,
}

impl JqlQueryWithUnknownUsers {
    /// JQL queries that contained users that could not be found
    pub fn new() -> JqlQueryWithUnknownUsers {
        JqlQueryWithUnknownUsers {
            converted_query: None,
            original_query: None,
        }
    }
}

