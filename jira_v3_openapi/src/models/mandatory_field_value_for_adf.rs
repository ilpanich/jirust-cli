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

/// MandatoryFieldValueForAdf : An object notation input
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MandatoryFieldValueForAdf {
    /// If `true`, will try to retain original non-null issue field values on move.
    #[serde(rename = "retain", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub retain: Option<Option<bool>>,
    /// Will treat as `MandatoryFieldValueForADF` if type is `adf`
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Value for each field. Accepts Atlassian Document Format (ADF) for rich text fields like `description`, `environments`. For ADF format details, refer to: [Atlassian Document Format](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure)
    #[serde(rename = "value")]
    pub value: serde_json::Value,
}

impl MandatoryFieldValueForAdf {
    /// An object notation input
    pub fn new(r#type: Type, value: serde_json::Value) -> MandatoryFieldValueForAdf {
        MandatoryFieldValueForAdf {
            retain: None,
            r#type,
            value,
        }
    }
}
/// Will treat as `MandatoryFieldValueForADF` if type is `adf`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "adf")]
    Adf,
    #[serde(rename = "raw")]
    Raw,
}

impl Default for Type {
    fn default() -> Type {
        Self::Adf
    }
}
