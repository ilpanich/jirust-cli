/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JiraExpressionsAnalysis : Details about the analysed Jira expression.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraExpressionsAnalysis {
    /// The results of Jira expressions analysis.
    #[serde(rename = "results")]
    pub results: Vec<models::JiraExpressionAnalysis>,
}

impl JiraExpressionsAnalysis {
    /// Details about the analysed Jira expression.
    pub fn new(results: Vec<models::JiraExpressionAnalysis>) -> JiraExpressionsAnalysis {
        JiraExpressionsAnalysis {
            results,
        }
    }
}
