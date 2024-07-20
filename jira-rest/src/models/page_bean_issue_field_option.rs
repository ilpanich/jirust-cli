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

/// PageBeanIssueFieldOption : A page of items.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageBeanIssueFieldOption {
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
    pub values: Option<Vec<models::IssueFieldOption>>,
}

impl PageBeanIssueFieldOption {
    /// A page of items.
    pub fn new() -> PageBeanIssueFieldOption {
        PageBeanIssueFieldOption {
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

