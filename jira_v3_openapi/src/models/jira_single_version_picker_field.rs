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
pub struct JiraSingleVersionPickerField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "version")]
    pub version: Box<models::JiraVersionField>,
}

impl JiraSingleVersionPickerField {
    pub fn new(field_id: String, version: models::JiraVersionField) -> JiraSingleVersionPickerField {
        JiraSingleVersionPickerField {
            field_id,
            version: Box::new(version),
        }
    }
}

