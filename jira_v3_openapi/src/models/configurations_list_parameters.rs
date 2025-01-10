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

/// ConfigurationsListParameters : List of custom fields identifiers which will be used to filter configurations
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationsListParameters {
    /// List of IDs or keys of the custom fields. It can be a mix of IDs and keys in the same query.
    #[serde(rename = "fieldIdsOrKeys")]
    pub field_ids_or_keys: Vec<String>,
}

impl ConfigurationsListParameters {
    /// List of custom fields identifiers which will be used to filter configurations
    pub fn new(field_ids_or_keys: Vec<String>) -> ConfigurationsListParameters {
        ConfigurationsListParameters {
            field_ids_or_keys,
        }
    }
}

