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

/// JExpEvaluateIssuesJqlMetaDataBean : The description of the page of issues loaded by the provided JQL query.This bean will be replacing IssuesJqlMetaDataBean bean as part of new `evaluate` endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JExpEvaluateIssuesJqlMetaDataBean {
    /// Next Page token for the next page of issues.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: String,
}

impl JExpEvaluateIssuesJqlMetaDataBean {
    /// The description of the page of issues loaded by the provided JQL query.This bean will be replacing IssuesJqlMetaDataBean bean as part of new `evaluate` endpoint
    pub fn new(next_page_token: String) -> JExpEvaluateIssuesJqlMetaDataBean {
        JExpEvaluateIssuesJqlMetaDataBean {
            next_page_token,
        }
    }
}

