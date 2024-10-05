/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SecuritySchemeMembersRequest : Details of issue security scheme level new members.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySchemeMembersRequest {
    /// The list of level members which should be added to the issue security scheme level.
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<models::SecuritySchemeLevelMemberBean>>,
}

impl SecuritySchemeMembersRequest {
    /// Details of issue security scheme level new members.
    pub fn new() -> SecuritySchemeMembersRequest {
        SecuritySchemeMembersRequest {
            members: None,
        }
    }
}

