/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
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

