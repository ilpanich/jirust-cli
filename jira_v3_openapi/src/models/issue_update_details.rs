/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueUpdateDetails : Details of an issue update request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueUpdateDetails {
    /// List of issue screen fields to update, specifying the sub-field to update and its value for each field. This field provides a straightforward option when setting a sub-field. When multiple sub-fields or other operations are required, use `update`. Fields included in here cannot be included in `update`.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Additional issue history details.
    #[serde(rename = "historyMetadata", skip_serializing_if = "Option::is_none")]
    pub history_metadata: Option<models::HistoryMetadata>,
    /// Details of issue properties to be add or update.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<models::EntityProperty>>,
    /// Details of a transition. Required when performing a transition, optional when creating or editing an issue.
    #[serde(rename = "transition", skip_serializing_if = "Option::is_none")]
    pub transition: Option<models::IssueTransition>,
    /// A Map containing the field field name and a list of operations to perform on the issue screen field. Note that fields included in here cannot be included in `fields`.
    #[serde(rename = "update", skip_serializing_if = "Option::is_none")]
    pub update: Option<std::collections::HashMap<String, Vec<models::FieldUpdateOperation>>>,
}

impl IssueUpdateDetails {
    /// Details of an issue update request.
    pub fn new() -> IssueUpdateDetails {
        IssueUpdateDetails {
            fields: None,
            history_metadata: None,
            properties: None,
            transition: None,
            update: None,
        }
    }
}

