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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RichText {
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "emptyAdf", skip_serializing_if = "Option::is_none")]
    pub empty_adf: Option<bool>,
    #[serde(rename = "finalised", skip_serializing_if = "Option::is_none")]
    pub finalised: Option<bool>,
    #[serde(rename = "valueSet", skip_serializing_if = "Option::is_none")]
    pub value_set: Option<bool>,
}

impl RichText {
    pub fn new() -> RichText {
        RichText {
            empty: None,
            empty_adf: None,
            finalised: None,
            value_set: None,
        }
    }
}

