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

/// ParsedJqlQueries : A list of parsed JQL queries.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParsedJqlQueries {
    /// A list of parsed JQL queries.
    #[serde(rename = "queries")]
    pub queries: Vec<models::ParsedJqlQuery>,
}

impl ParsedJqlQueries {
    /// A list of parsed JQL queries.
    pub fn new(queries: Vec<models::ParsedJqlQuery>) -> ParsedJqlQueries {
        ParsedJqlQueries {
            queries,
        }
    }
}

