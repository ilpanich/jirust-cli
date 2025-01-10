/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EntityProperty : An entity property, for more information see [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityProperty {
    /// The key of the property. Required on create and update.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The value of the property. Required on create and update.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<serde_json::Value>>,
}

impl EntityProperty {
    /// An entity property, for more information see [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
    pub fn new() -> EntityProperty {
        EntityProperty {
            key: None,
            value: None,
        }
    }
}

