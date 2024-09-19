/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JqlQueryField : A field used in a JQL query. See [Advanced searching - fields reference](https://confluence.atlassian.com/x/dAiiLQ) for more information about fields in JQL queries.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryField {
    /// The encoded name of the field, which can be used directly in a JQL query.
    #[serde(rename = "encodedName", skip_serializing_if = "Option::is_none")]
    pub encoded_name: Option<String>,
    /// The name of the field.
    #[serde(rename = "name")]
    pub name: String,
    /// When the field refers to a value in an entity property, details of the entity property value.
    #[serde(rename = "property", skip_serializing_if = "Option::is_none")]
    pub property: Option<Vec<models::JqlQueryFieldEntityProperty>>,
}

impl JqlQueryField {
    /// A field used in a JQL query. See [Advanced searching - fields reference](https://confluence.atlassian.com/x/dAiiLQ) for more information about fields in JQL queries.
    pub fn new(name: String) -> JqlQueryField {
        JqlQueryField {
            encoded_name: None,
            name,
            property: None,
        }
    }
}

