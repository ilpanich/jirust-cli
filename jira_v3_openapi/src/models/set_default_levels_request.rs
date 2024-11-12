/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SetDefaultLevelsRequest : Details of new default levels.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetDefaultLevelsRequest {
    /// List of objects with issue security scheme ID and new default level ID.
    #[serde(rename = "defaultValues")]
    pub default_values: Vec<models::DefaultLevelValue>,
}

impl SetDefaultLevelsRequest {
    /// Details of new default levels.
    pub fn new(default_values: Vec<models::DefaultLevelValue>) -> SetDefaultLevelsRequest {
        SetDefaultLevelsRequest {
            default_values,
        }
    }
}

