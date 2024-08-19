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

/// PageBeanContextForProjectAndIssueType : A page of items.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageBeanContextForProjectAndIssueType {
    /// Whether this is the last page.
    #[serde(rename = "isLast", skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
    /// The maximum number of items that could be returned.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// If there is another page of results, the URL of the next page.
    #[serde(rename = "nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    /// The URL of the page.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The index of the first item returned.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The number of items returned.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// The list of items.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::ContextForProjectAndIssueType>>,
}

impl PageBeanContextForProjectAndIssueType {
    /// A page of items.
    pub fn new() -> PageBeanContextForProjectAndIssueType {
        PageBeanContextForProjectAndIssueType {
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

