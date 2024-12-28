/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssuesJqlMetaDataBean : The description of the page of issues loaded by the provided JQL query.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuesJqlMetaDataBean {
    /// The number of issues that were loaded in this evaluation.
    #[serde(rename = "count")]
    pub count: i32,
    /// The maximum number of issues that could be loaded in this evaluation.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// The index of the first issue.
    #[serde(rename = "startAt")]
    pub start_at: i64,
    /// The total number of issues the JQL returned.
    #[serde(rename = "totalCount")]
    pub total_count: i64,
    /// Any warnings related to the JQL query. Present only if the validation mode was set to `warn`.
    #[serde(rename = "validationWarnings", skip_serializing_if = "Option::is_none")]
    pub validation_warnings: Option<Vec<String>>,
}

impl IssuesJqlMetaDataBean {
    /// The description of the page of issues loaded by the provided JQL query.
    pub fn new(count: i32, max_results: i32, start_at: i64, total_count: i64) -> IssuesJqlMetaDataBean {
        IssuesJqlMetaDataBean {
            count,
            max_results,
            start_at,
            total_count,
            validation_warnings: None,
        }
    }
}

