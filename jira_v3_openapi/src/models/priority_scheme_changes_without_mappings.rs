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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrioritySchemeChangesWithoutMappings {
    /// Affected entity ids.
    #[serde(rename = "ids")]
    pub ids: Vec<i64>,
}

impl PrioritySchemeChangesWithoutMappings {
    pub fn new(ids: Vec<i64>) -> PrioritySchemeChangesWithoutMappings {
        PrioritySchemeChangesWithoutMappings {
            ids,
        }
    }
}

