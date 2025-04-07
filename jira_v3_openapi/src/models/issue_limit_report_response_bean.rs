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
pub struct IssueLimitReportResponseBean {
    /// A list of ids of issues approaching the limit and their field count
    #[serde(rename = "issuesApproachingLimit", skip_serializing_if = "Option::is_none")]
    pub issues_approaching_limit: Option<std::collections::HashMap<String, std::collections::HashMap<String, i64>>>,
    /// A list of ids of issues breaching the limit and their field count
    #[serde(rename = "issuesBreachingLimit", skip_serializing_if = "Option::is_none")]
    pub issues_breaching_limit: Option<std::collections::HashMap<String, std::collections::HashMap<String, i64>>>,
    /// The fields and their defined limits
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<std::collections::HashMap<String, i32>>,
}

impl IssueLimitReportResponseBean {
    pub fn new() -> IssueLimitReportResponseBean {
        IssueLimitReportResponseBean {
            issues_approaching_limit: None,
            issues_breaching_limit: None,
            limits: None,
        }
    }
}

