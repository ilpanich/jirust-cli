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

/// JiraSingleSelectField : Add or clear a single select field:   *  To add, specify the option with an `optionId`.  *  To clear, pass an option with `optionId` as `-1`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraSingleSelectField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "option")]
    pub option: Box<models::JiraSelectedOptionField>,
}

impl JiraSingleSelectField {
    /// Add or clear a single select field:   *  To add, specify the option with an `optionId`.  *  To clear, pass an option with `optionId` as `-1`.
    pub fn new(field_id: String, option: models::JiraSelectedOptionField) -> JiraSingleSelectField {
        JiraSingleSelectField {
            field_id,
            option: Box::new(option),
        }
    }
}

