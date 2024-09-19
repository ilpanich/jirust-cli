/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlCountRequestBean {
    /// A [JQL](https://confluence.atlassian.com/x/egORLQ) expression. For performance reasons, this field requires a bounded query. A bounded query is a query with a search restriction.
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
}

impl JqlCountRequestBean {
    pub fn new() -> JqlCountRequestBean {
        JqlCountRequestBean {
            jql: None,
        }
    }
}

