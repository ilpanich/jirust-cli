/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PagedListUserDetailsApplicationUser : A paged list. To access additional details append `[start-index:end-index]` to the expand request. For example, `?expand=sharedUsers[10:40]` returns a list starting at item 10 and finishing at item 40.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedListUserDetailsApplicationUser {
    /// The index of the last item returned on the page.
    #[serde(rename = "end-index", skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
    /// The list of items.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::UserDetails>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "max-results", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The number of items on the page.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// The index of the first item returned on the page.
    #[serde(rename = "start-index", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}

impl PagedListUserDetailsApplicationUser {
    /// A paged list. To access additional details append `[start-index:end-index]` to the expand request. For example, `?expand=sharedUsers[10:40]` returns a list starting at item 10 and finishing at item 40.
    pub fn new() -> PagedListUserDetailsApplicationUser {
        PagedListUserDetailsApplicationUser {
            end_index: None,
            items: None,
            max_results: None,
            size: None,
            start_index: None,
        }
    }
}

