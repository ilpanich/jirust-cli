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

/// FieldReferenceData : Details of a field that can be used in advanced searches.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldReferenceData {
    /// Whether the field provide auto-complete suggestions.
    #[serde(rename = "auto", skip_serializing_if = "Option::is_none")]
    pub auto: Option<Auto>,
    /// If the item is a custom field, the ID of the custom field.
    #[serde(rename = "cfid", skip_serializing_if = "Option::is_none")]
    pub cfid: Option<String>,
    /// Whether this field has been deprecated.
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<Deprecated>,
    /// The searcher key of the field, only passed when the field is deprecated.
    #[serde(rename = "deprecatedSearcherKey", skip_serializing_if = "Option::is_none")]
    pub deprecated_searcher_key: Option<String>,
    /// The display name contains the following:   *  for system fields, the field name. For example, `Summary`.  *  for collapsed custom fields, the field name followed by a hyphen and then the field name and field type. For example, `Component - Component[Dropdown]`.  *  for other custom fields, the field name followed by a hyphen and then the custom field ID. For example, `Component - cf[10061]`.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The valid search operators for the field.
    #[serde(rename = "operators", skip_serializing_if = "Option::is_none")]
    pub operators: Option<Vec<String>>,
    /// Whether the field can be used in a query's `ORDER BY` clause.
    #[serde(rename = "orderable", skip_serializing_if = "Option::is_none")]
    pub orderable: Option<Orderable>,
    /// Whether the content of this field can be searched.
    #[serde(rename = "searchable", skip_serializing_if = "Option::is_none")]
    pub searchable: Option<Searchable>,
    /// The data types of items in the field.
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// The field identifier.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl FieldReferenceData {
    /// Details of a field that can be used in advanced searches.
    pub fn new() -> FieldReferenceData {
        FieldReferenceData {
            auto: None,
            cfid: None,
            deprecated: None,
            deprecated_searcher_key: None,
            display_name: None,
            operators: None,
            orderable: None,
            searchable: None,
            types: None,
            value: None,
        }
    }
}
/// Whether the field provide auto-complete suggestions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Auto {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for Auto {
    fn default() -> Auto {
        Self::True
    }
}
/// Whether this field has been deprecated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Deprecated {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for Deprecated {
    fn default() -> Deprecated {
        Self::True
    }
}
/// Whether the field can be used in a query's `ORDER BY` clause.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Orderable {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for Orderable {
    fn default() -> Orderable {
        Self::True
    }
}
/// Whether the content of this field can be searched.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Searchable {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for Searchable {
    fn default() -> Searchable {
        Self::True
    }
}

