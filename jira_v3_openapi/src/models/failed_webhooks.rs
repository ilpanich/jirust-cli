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

/// FailedWebhooks : A page of failed webhooks.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedWebhooks {
    /// The maximum number of items on the page. If the list of values is shorter than this number, then there are no more pages.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// The URL to the next page of results. Present only if the request returned at least one result.The next page may be empty at the time of receiving the response, but new failed webhooks may appear in time. You can save the URL to the next page and query for new results periodically (for example, every hour).
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// The list of webhooks.
    #[serde(rename = "values")]
    pub values: Vec<models::FailedWebhook>,
}

impl FailedWebhooks {
    /// A page of failed webhooks.
    pub fn new(max_results: i32, values: Vec<models::FailedWebhook>) -> FailedWebhooks {
        FailedWebhooks {
            max_results,
            next: None,
            values,
        }
    }
}

