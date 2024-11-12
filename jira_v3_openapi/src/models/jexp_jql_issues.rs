/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JexpJqlIssues : The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable. Not all issues returned by the JQL query are loaded, only those described by the `startAt` and `maxResults` properties. To determine whether it is necessary to iterate to ensure all the issues returned by the JQL query are evaluated, inspect `meta.issues.jql.count` in the response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JexpJqlIssues {
    /// The maximum number of issues to return from the JQL query. Inspect `meta.issues.jql.maxResults` in the response to ensure the maximum value has not been exceeded.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The JQL query.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// The index of the first issue to return from the JQL query.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// Determines how to validate the JQL query and treat the validation results.
    #[serde(rename = "validation", skip_serializing_if = "Option::is_none")]
    pub validation: Option<Validation>,
}

impl JexpJqlIssues {
    /// The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable. Not all issues returned by the JQL query are loaded, only those described by the `startAt` and `maxResults` properties. To determine whether it is necessary to iterate to ensure all the issues returned by the JQL query are evaluated, inspect `meta.issues.jql.count` in the response.
    pub fn new() -> JexpJqlIssues {
        JexpJqlIssues {
            max_results: None,
            query: None,
            start_at: None,
            validation: None,
        }
    }
}
/// Determines how to validate the JQL query and treat the validation results.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Validation {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "none")]
    None,
}

impl Default for Validation {
    fn default() -> Validation {
        Self::Strict
    }
}

