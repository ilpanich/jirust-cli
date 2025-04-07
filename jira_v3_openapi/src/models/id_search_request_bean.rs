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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdSearchRequestBean {
    /// A [JQL](https://confluence.atlassian.com/x/egORLQ) expression. Order by clauses are not allowed.
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    /// The maximum number of items to return per page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The continuation token to fetch the next page. This token is provided by the response of this endpoint.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl IdSearchRequestBean {
    pub fn new() -> IdSearchRequestBean {
        IdSearchRequestBean {
            jql: None,
            max_results: None,
            next_page_token: None,
        }
    }
}

