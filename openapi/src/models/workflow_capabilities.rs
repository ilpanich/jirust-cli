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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCapabilities {
    /// The Connect provided ecosystem rules available.
    #[serde(rename = "connectRules", skip_serializing_if = "Option::is_none")]
    pub connect_rules: Option<Vec<models::AvailableWorkflowConnectRule>>,
    /// The scope of the workflow capabilities. `GLOBAL` for company-managed projects and `PROJECT` for team-managed projects.
    #[serde(rename = "editorScope", skip_serializing_if = "Option::is_none")]
    pub editor_scope: Option<EditorScope>,
    /// The Forge provided ecosystem rules available.
    #[serde(rename = "forgeRules", skip_serializing_if = "Option::is_none")]
    pub forge_rules: Option<Vec<models::AvailableWorkflowForgeRule>>,
    /// The types of projects that this capability set is available for.
    #[serde(rename = "projectTypes", skip_serializing_if = "Option::is_none")]
    pub project_types: Option<Vec<ProjectTypes>>,
    /// The Atlassian provided system rules available.
    #[serde(rename = "systemRules", skip_serializing_if = "Option::is_none")]
    pub system_rules: Option<Vec<models::AvailableWorkflowSystemRule>>,
    /// The trigger rules available.
    #[serde(rename = "triggerRules", skip_serializing_if = "Option::is_none")]
    pub trigger_rules: Option<Vec<models::AvailableWorkflowTriggers>>,
}

impl WorkflowCapabilities {
    pub fn new() -> WorkflowCapabilities {
        WorkflowCapabilities {
            connect_rules: None,
            editor_scope: None,
            forge_rules: None,
            project_types: None,
            system_rules: None,
            trigger_rules: None,
        }
    }
}
/// The scope of the workflow capabilities. `GLOBAL` for company-managed projects and `PROJECT` for team-managed projects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EditorScope {
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "GLOBAL")]
    Global,
}

impl Default for EditorScope {
    fn default() -> EditorScope {
        Self::Project
    }
}
/// The types of projects that this capability set is available for.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectTypes {
    #[serde(rename = "software")]
    Software,
    #[serde(rename = "service_desk")]
    ServiceDesk,
    #[serde(rename = "product_discovery")]
    ProductDiscovery,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for ProjectTypes {
    fn default() -> ProjectTypes {
        Self::Software
    }
}

