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

/// IdSearchResults : Result of your JQL search. Returns a list of issue IDs and a token to fetch the next page if one exists.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdSearchResults {
    /// The list of issue IDs found by the search.
    #[serde(rename = "issueIds", skip_serializing_if = "Option::is_none")]
    pub issue_ids: Option<Vec<i64>>,
    /// Continuation token to fetch the next page. If this result represents the last or the only page this token will be null.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl IdSearchResults {
    /// Result of your JQL search. Returns a list of issue IDs and a token to fetch the next page if one exists.
    pub fn new() -> IdSearchResults {
        IdSearchResults {
            issue_ids: None,
            next_page_token: None,
        }
    }
}

