/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueMatchesForJql : A list of the issues matched to a JQL query or details of errors encountered during matching.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueMatchesForJql {
    /// A list of errors.
    #[serde(rename = "errors")]
    pub errors: Vec<String>,
    /// A list of issue IDs.
    #[serde(rename = "matchedIssues")]
    pub matched_issues: Vec<i64>,
}

impl IssueMatchesForJql {
    /// A list of the issues matched to a JQL query or details of errors encountered during matching.
    pub fn new(errors: Vec<String>, matched_issues: Vec<i64>) -> IssueMatchesForJql {
        IssueMatchesForJql {
            errors,
            matched_issues,
        }
    }
}

