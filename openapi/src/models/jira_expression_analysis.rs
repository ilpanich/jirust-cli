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

/// JiraExpressionAnalysis : Details about the analysed Jira expression.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraExpressionAnalysis {
    #[serde(rename = "complexity", skip_serializing_if = "Option::is_none")]
    pub complexity: Option<Box<models::JiraExpressionComplexity>>,
    /// A list of validation errors. Not included if the expression is valid.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::JiraExpressionValidationError>>,
    /// The analysed expression.
    #[serde(rename = "expression")]
    pub expression: String,
    /// EXPERIMENTAL. The inferred type of the expression.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Whether the expression is valid and the interpreter will evaluate it. Note that the expression may fail at runtime (for example, if it executes too many expensive operations).
    #[serde(rename = "valid")]
    pub valid: bool,
}

impl JiraExpressionAnalysis {
    /// Details about the analysed Jira expression.
    pub fn new(expression: String, valid: bool) -> JiraExpressionAnalysis {
        JiraExpressionAnalysis {
            complexity: None,
            errors: None,
            expression,
            r#type: None,
            valid,
        }
    }
}

