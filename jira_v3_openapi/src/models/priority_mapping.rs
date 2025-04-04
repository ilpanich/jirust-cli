/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PriorityMapping : Mapping of issue priorities for changes in priority schemes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriorityMapping {
    /// The mapping of priorities for issues being migrated **into** this priority scheme. Key is the old priority ID, value is the new priority ID (must exist in this priority scheme).  E.g. The current priority scheme has priority ID `10001`. Issues with priority ID `10000` are being migrated into this priority scheme will need mapping to new priorities. The `in` mapping would be `{\"10000\": 10001}`.
    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    pub r#in: Option<std::collections::HashMap<String, i64>>,
    /// The mapping of priorities for issues being migrated **out of** this priority scheme. Key is the old priority ID (must exist in this priority scheme), value is the new priority ID (must exist in the default priority scheme). Required for updating an existing priority scheme. Not used when creating a new priority scheme.  E.g. The current priority scheme has priority ID `10001`. Issues with priority ID `10001` are being migrated out of this priority scheme will need mapping to new priorities. The `out` mapping would be `{\"10001\": 10000}`.
    #[serde(rename = "out", skip_serializing_if = "Option::is_none")]
    pub out: Option<std::collections::HashMap<String, i64>>,
}

impl PriorityMapping {
    /// Mapping of issue priorities for changes in priority schemes.
    pub fn new() -> PriorityMapping {
        PriorityMapping {
            r#in: None,
            out: None,
        }
    }
}

