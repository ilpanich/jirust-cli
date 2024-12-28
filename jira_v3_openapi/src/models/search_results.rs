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

/// SearchResults : The result of a JQL search.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResults {
    /// Expand options that include additional search result details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The list of issues found by the search.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<models::IssueBean>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The ID and name of each field in the search results.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<std::collections::HashMap<String, String>>,
    /// The schema describing the field types in the search results.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::collections::HashMap<String, models::JsonTypeBean>>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    /// The number of results on the page.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// Any warnings related to the JQL query.
    #[serde(rename = "warningMessages", skip_serializing_if = "Option::is_none")]
    pub warning_messages: Option<Vec<String>>,
}

impl SearchResults {
    /// The result of a JQL search.
    pub fn new() -> SearchResults {
        SearchResults {
            expand: None,
            issues: None,
            max_results: None,
            names: None,
            schema: None,
            start_at: None,
            total: None,
            warning_messages: None,
        }
    }
}

