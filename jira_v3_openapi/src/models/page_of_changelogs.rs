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

/// PageOfChangelogs : A page of changelogs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageOfChangelogs {
    /// The list of changelogs.
    #[serde(rename = "histories", skip_serializing_if = "Option::is_none")]
    pub histories: Option<Vec<models::Changelog>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    /// The number of results on the page.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl PageOfChangelogs {
    /// A page of changelogs.
    pub fn new() -> PageOfChangelogs {
        PageOfChangelogs {
            histories: None,
            max_results: None,
            start_at: None,
            total: None,
        }
    }
}

