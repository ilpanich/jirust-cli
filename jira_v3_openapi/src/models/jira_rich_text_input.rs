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
pub struct JiraRichTextInput {
    #[serde(rename = "adfValue", skip_serializing_if = "Option::is_none")]
    pub adf_value: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl JiraRichTextInput {
    pub fn new() -> JiraRichTextInput {
        JiraRichTextInput {
            adf_value: None,
        }
    }
}

