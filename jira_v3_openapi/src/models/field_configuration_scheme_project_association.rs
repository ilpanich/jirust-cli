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

/// FieldConfigurationSchemeProjectAssociation : Associated field configuration scheme and project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfigurationSchemeProjectAssociation {
    /// The ID of the field configuration scheme. If the field configuration scheme ID is `null`, the operation assigns the default field configuration scheme.
    #[serde(rename = "fieldConfigurationSchemeId", skip_serializing_if = "Option::is_none")]
    pub field_configuration_scheme_id: Option<String>,
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
}

impl FieldConfigurationSchemeProjectAssociation {
    /// Associated field configuration scheme and project.
    pub fn new(project_id: String) -> FieldConfigurationSchemeProjectAssociation {
        FieldConfigurationSchemeProjectAssociation {
            field_configuration_scheme_id: None,
            project_id,
        }
    }
}

