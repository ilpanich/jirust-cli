/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DataClassificationTagBean : The data classification.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataClassificationTagBean {
    /// The color of the data classification object.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The description of the data classification object.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The guideline of the data classification object.
    #[serde(rename = "guideline", skip_serializing_if = "Option::is_none")]
    pub guideline: Option<String>,
    /// The ID of the data classification object.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the data classification object.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The rank of the data classification object.
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// The status of the data classification object.
    #[serde(rename = "status")]
    pub status: String,
}

impl DataClassificationTagBean {
    /// The data classification.
    pub fn new(id: String, status: String) -> DataClassificationTagBean {
        DataClassificationTagBean {
            color: None,
            description: None,
            guideline: None,
            id,
            name: None,
            rank: None,
            status,
        }
    }
}

