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

/// FieldDetails : Details about a field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldDetails {
    /// The names that can be used to reference the field in an advanced search. For more information, see [Advanced searching - fields reference](https://confluence.atlassian.com/x/gwORLQ).
    #[serde(rename = "clauseNames", skip_serializing_if = "Option::is_none")]
    pub clause_names: Option<Vec<String>>,
    /// Whether the field is a custom field.
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    /// The ID of the field.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the field.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the field.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the field can be used as a column on the issue navigator.
    #[serde(rename = "navigable", skip_serializing_if = "Option::is_none")]
    pub navigable: Option<bool>,
    /// Whether the content of the field can be used to order lists.
    #[serde(rename = "orderable", skip_serializing_if = "Option::is_none")]
    pub orderable: Option<bool>,
    /// The data schema for the field.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<models::JsonTypeBean>>,
    /// The scope of the field.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::Scope>,
    /// Whether the content of the field can be searched.
    #[serde(rename = "searchable", skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
}

impl FieldDetails {
    /// Details about a field.
    pub fn new() -> FieldDetails {
        FieldDetails {
            clause_names: None,
            custom: None,
            id: None,
            key: None,
            name: None,
            navigable: None,
            orderable: None,
            schema: None,
            scope: None,
            searchable: None,
        }
    }
}

