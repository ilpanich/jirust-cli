/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JExpEvaluateIssuesMetaBean : Meta data describing the `issues` context variable.This bean will be replacing IssuesMetaBean bean as part of new `evaluate` endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JExpEvaluateIssuesMetaBean {
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<Box<models::JExpEvaluateIssuesJqlMetaDataBean>>,
}

impl JExpEvaluateIssuesMetaBean {
    /// Meta data describing the `issues` context variable.This bean will be replacing IssuesMetaBean bean as part of new `evaluate` endpoint
    pub fn new() -> JExpEvaluateIssuesMetaBean {
        JExpEvaluateIssuesMetaBean {
            jql: None,
        }
    }
}

