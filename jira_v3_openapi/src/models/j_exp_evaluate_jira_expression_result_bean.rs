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

/// JExpEvaluateJiraExpressionResultBean : The result of evaluating a Jira expression.This bean will be replacing `JiraExpressionResultBean` bean as part of new evaluate endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JExpEvaluateJiraExpressionResultBean {
    /// Contains various characteristics of the performed expression evaluation.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::JExpEvaluateMetaDataBean>>,
    /// The value of the evaluated expression. It may be a primitive JSON value or a Jira REST API object. (Some expressions do not produce any meaningful results—for example, an expression that returns a lambda function—if that's the case a simple string representation is returned. These string representations should not be relied upon and may change without notice.)
    #[serde(rename = "value", deserialize_with = "Option::deserialize")]
    pub value: Option<serde_json::Value>,
}

impl JExpEvaluateJiraExpressionResultBean {
    /// The result of evaluating a Jira expression.This bean will be replacing `JiraExpressionResultBean` bean as part of new evaluate endpoint
    pub fn new(value: Option<serde_json::Value>) -> JExpEvaluateJiraExpressionResultBean {
        JExpEvaluateJiraExpressionResultBean {
            meta: None,
            value,
        }
    }
}

