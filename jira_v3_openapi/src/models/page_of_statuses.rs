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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageOfStatuses {
    /// Whether this is the last page.
    #[serde(rename = "isLast", skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
    /// The maximum number of items that could be returned.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The URL of the next page of results, if any.
    #[serde(rename = "nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    /// The URL of this page.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// Number of items that satisfy the search.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// The list of items.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::JiraStatus>>,
}

impl PageOfStatuses {
    pub fn new() -> PageOfStatuses {
        PageOfStatuses {
            is_last: None,
            max_results: None,
            next_page: None,
            param_self: None,
            start_at: None,
            total: None,
            values: None,
        }
    }
}

