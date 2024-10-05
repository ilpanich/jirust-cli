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

/// JqlQueryFieldEntityProperty : Details of an entity property.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryFieldEntityProperty {
    /// The object on which the property is set.
    #[serde(rename = "entity")]
    pub entity: String,
    /// The key of the property.
    #[serde(rename = "key")]
    pub key: String,
    /// The path in the property value to query.
    #[serde(rename = "path")]
    pub path: String,
    /// The type of the property value extraction. Not available if the extraction for the property is not registered on the instance with the [Entity property](https://developer.atlassian.com/cloud/jira/platform/modules/entity-property/) module.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl JqlQueryFieldEntityProperty {
    /// Details of an entity property.
    pub fn new(entity: String, key: String, path: String) -> JqlQueryFieldEntityProperty {
        JqlQueryFieldEntityProperty {
            entity,
            key,
            path,
            r#type: None,
        }
    }
}
/// The type of the property value extraction. Not available if the extraction for the property is not registered on the instance with the [Entity property](https://developer.atlassian.com/cloud/jira/platform/modules/entity-property/) module.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "user")]
    User,
}

impl Default for Type {
    fn default() -> Type {
        Self::Number
    }
}

