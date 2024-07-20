/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// MultipleCustomFieldValuesUpdate : A custom field and its new value with a list of issue to update.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultipleCustomFieldValuesUpdate {
    /// The ID or key of the custom field. For example, `customfield_10010`.
    #[serde(rename = "customField")]
    pub custom_field: String,
    /// The list of issue IDs.
    #[serde(rename = "issueIds")]
    pub issue_ids: Vec<i64>,
    /// The value for the custom field. The value must be compatible with the [custom field type](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/#data-types) as follows:   *  `string` the value must be a string.  *  `number` the value must be a number.  *  `datetime` the value must be a string that represents a date in the ISO format or the simplified extended ISO format. For example, `\"2023-01-18T12:00:00-03:00\"` or `\"2023-01-18T12:00:00.000Z\"`. However, the milliseconds part is ignored.  *  `user` the value must be an object that contains the `accountId` field.  *  `group` the value must be an object that contains the group `name` or `groupId` field. Because group names can change, we recommend using `groupId`.  A list of appropriate values must be provided if the field is of the `list` [collection type](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/#collection-types).
    #[serde(rename = "value", deserialize_with = "Option::deserialize")]
    pub value: Option<serde_json::Value>,
}

impl MultipleCustomFieldValuesUpdate {
    /// A custom field and its new value with a list of issue to update.
    pub fn new(custom_field: String, issue_ids: Vec<i64>, value: Option<serde_json::Value>) -> MultipleCustomFieldValuesUpdate {
        MultipleCustomFieldValuesUpdate {
            custom_field,
            issue_ids,
            value,
        }
    }
}

