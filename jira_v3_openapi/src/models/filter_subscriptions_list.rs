/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FilterSubscriptionsList : A paginated list of subscriptions to a filter.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterSubscriptionsList {
    /// The index of the last item returned on the page.
    #[serde(rename = "end-index", skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
    /// The list of items.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::FilterSubscription>>,
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

impl FilterSubscriptionsList {
    /// A paginated list of subscriptions to a filter.
    pub fn new() -> FilterSubscriptionsList {
        FilterSubscriptionsList {
            end_index: None,
            items: None,
            max_results: None,
            size: None,
            start_index: None,
        }
    }
}

