/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DataClassificationLevelsBean : The data classification.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataClassificationLevelsBean {
    /// The data classifications.
    #[serde(rename = "classifications", skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Vec<models::DataClassificationTagBean>>,
}

impl DataClassificationLevelsBean {
    /// The data classification.
    pub fn new() -> DataClassificationLevelsBean {
        DataClassificationLevelsBean {
            classifications: None,
        }
    }
}

