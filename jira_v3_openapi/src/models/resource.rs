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
pub struct Resource {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<std::path::PathBuf>,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "inputStream", skip_serializing_if = "Option::is_none")]
    pub input_stream: Option<serde_json::Value>,
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    #[serde(rename = "readable", skip_serializing_if = "Option::is_none")]
    pub readable: Option<bool>,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            description: None,
            file: None,
            filename: None,
            input_stream: None,
            open: None,
            readable: None,
            uri: None,
            url: None,
        }
    }
}

