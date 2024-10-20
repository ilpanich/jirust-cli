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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraUrlField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl JiraUrlField {
    pub fn new(field_id: String, url: String) -> JiraUrlField {
        JiraUrlField {
            field_id,
            url,
        }
    }
}

