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

/// JiraExpressionForAnalysis : Details of Jira expressions for analysis.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraExpressionForAnalysis {
    /// Context variables and their types. The type checker assumes that [common context variables](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/#context-variables), such as `issue` or `project`, are available in context and sets their type. Use this property to override the default types or provide details of new variables.
    #[serde(rename = "contextVariables", skip_serializing_if = "Option::is_none")]
    pub context_variables: Option<std::collections::HashMap<String, String>>,
    /// The list of Jira expressions to analyse.
    #[serde(rename = "expressions")]
    pub expressions: Vec<String>,
}

impl JiraExpressionForAnalysis {
    /// Details of Jira expressions for analysis.
    pub fn new(expressions: Vec<String>) -> JiraExpressionForAnalysis {
        JiraExpressionForAnalysis {
            context_variables: None,
            expressions,
        }
    }
}

