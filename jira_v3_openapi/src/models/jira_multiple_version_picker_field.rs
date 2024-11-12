/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraMultipleVersionPickerField {
    #[serde(rename = "bulkEditMultiSelectFieldOption")]
    pub bulk_edit_multi_select_field_option: BulkEditMultiSelectFieldOption,
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "versions")]
    pub versions: Vec<models::JiraVersionField>,
}

impl JiraMultipleVersionPickerField {
    pub fn new(bulk_edit_multi_select_field_option: BulkEditMultiSelectFieldOption, field_id: String, versions: Vec<models::JiraVersionField>) -> JiraMultipleVersionPickerField {
        JiraMultipleVersionPickerField {
            bulk_edit_multi_select_field_option,
            field_id,
            versions,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BulkEditMultiSelectFieldOption {
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "REMOVE")]
    Remove,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "REMOVE_ALL")]
    RemoveAll,
}

impl Default for BulkEditMultiSelectFieldOption {
    fn default() -> BulkEditMultiSelectFieldOption {
        Self::Add
    }
}

