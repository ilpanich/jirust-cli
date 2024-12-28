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
pub struct JiraLabelsField {
    #[serde(rename = "bulkEditMultiSelectFieldOption")]
    pub bulk_edit_multi_select_field_option: BulkEditMultiSelectFieldOption,
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "labels")]
    pub labels: Vec<models::JiraLabelsInput>,
}

impl JiraLabelsField {
    pub fn new(bulk_edit_multi_select_field_option: BulkEditMultiSelectFieldOption, field_id: String, labels: Vec<models::JiraLabelsInput>) -> JiraLabelsField {
        JiraLabelsField {
            bulk_edit_multi_select_field_option,
            field_id,
            labels,
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

