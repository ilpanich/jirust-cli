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

/// CreateIssueSecuritySchemeDetails : Issue security scheme and it's details
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIssueSecuritySchemeDetails {
    /// The description of the issue security scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The list of scheme levels which should be added to the security scheme.
    #[serde(rename = "levels", skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<models::SecuritySchemeLevelBean>>,
    /// The name of the issue security scheme. Must be unique (case-insensitive).
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateIssueSecuritySchemeDetails {
    /// Issue security scheme and it's details
    pub fn new(name: String) -> CreateIssueSecuritySchemeDetails {
        CreateIssueSecuritySchemeDetails {
            description: None,
            levels: None,
            name,
        }
    }
}

