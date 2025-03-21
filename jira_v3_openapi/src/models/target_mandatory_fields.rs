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

/// TargetMandatoryFields : Field mapping for mandatory fields in target
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetMandatoryFields {
    /// Contains the value of mandatory fields
    #[serde(rename = "fields")]
    pub fields: std::collections::HashMap<String, models::Fields>,
}

impl TargetMandatoryFields {
    /// Field mapping for mandatory fields in target
    pub fn new(fields: std::collections::HashMap<String, models::Fields>) -> TargetMandatoryFields {
        TargetMandatoryFields {
            fields,
        }
    }
}

