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

/// ProjectRole : Details about the roles in a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRole {
    /// The list of users who act in this role.
    #[serde(rename = "actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<models::RoleActor>>,
    /// Whether this role is the admin role for the project.
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    /// Whether the calling user is part of this role.
    #[serde(rename = "currentUserRole", skip_serializing_if = "Option::is_none")]
    pub current_user_role: Option<bool>,
    /// Whether this role is the default role for the project
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// The description of the project role.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the project role.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the project role.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the roles are configurable for this project.
    #[serde(rename = "roleConfigurable", skip_serializing_if = "Option::is_none")]
    pub role_configurable: Option<bool>,
    /// The scope of the role. Indicated for roles associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO).
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::Scope>,
    /// The URL the project role details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The translated name of the project role.
    #[serde(rename = "translatedName", skip_serializing_if = "Option::is_none")]
    pub translated_name: Option<String>,
}

impl ProjectRole {
    /// Details about the roles in a project.
    pub fn new() -> ProjectRole {
        ProjectRole {
            actors: None,
            admin: None,
            current_user_role: None,
            default: None,
            description: None,
            id: None,
            name: None,
            role_configurable: None,
            scope: None,
            param_self: None,
            translated_name: None,
        }
    }
}

