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
pub struct JiraExpressionEvaluationMetaDataBean {
    /// Contains information about the expression complexity. For example, the number of steps it took to evaluate the expression.
    #[serde(rename = "complexity", skip_serializing_if = "Option::is_none")]
    pub complexity: Option<Box<models::JiraExpressionsComplexityBean>>,
    /// Contains information about the `issues` variable in the context. For example, is the issues were loaded with JQL, information about the page will be included here.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Box<models::IssuesMetaBean>>,
}

impl JiraExpressionEvaluationMetaDataBean {
    pub fn new() -> JiraExpressionEvaluationMetaDataBean {
        JiraExpressionEvaluationMetaDataBean {
            complexity: None,
            issues: None,
        }
    }
}

