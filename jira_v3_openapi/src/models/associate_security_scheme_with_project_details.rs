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

/// AssociateSecuritySchemeWithProjectDetails : Issue security scheme, project, and remapping details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssociateSecuritySchemeWithProjectDetails {
    /// The list of scheme levels which should be remapped to new levels of the issue security scheme.
    #[serde(rename = "oldToNewSecurityLevelMappings", skip_serializing_if = "Option::is_none")]
    pub old_to_new_security_level_mappings: Option<Vec<models::OldToNewSecurityLevelMappingsBean>>,
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// The ID of the issue security scheme. Providing null will clear the association with the issue security scheme.
    #[serde(rename = "schemeId")]
    pub scheme_id: String,
}

impl AssociateSecuritySchemeWithProjectDetails {
    /// Issue security scheme, project, and remapping details.
    pub fn new(project_id: String, scheme_id: String) -> AssociateSecuritySchemeWithProjectDetails {
        AssociateSecuritySchemeWithProjectDetails {
            old_to_new_security_level_mappings: None,
            project_id,
            scheme_id,
        }
    }
}

