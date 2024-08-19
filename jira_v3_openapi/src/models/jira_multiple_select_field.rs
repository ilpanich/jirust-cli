/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraMultipleSelectField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "options")]
    pub options: Vec<models::JiraSelectedOptionField>,
}

impl JiraMultipleSelectField {
    pub fn new(field_id: String, options: Vec<models::JiraSelectedOptionField>) -> JiraMultipleSelectField {
        JiraMultipleSelectField {
            field_id,
            options,
        }
    }
}

