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
pub struct ProjectRoleActorsUpdateBean {
    /// The actors to add to the project role.  Add groups using:   *  `atlassian-group-role-actor` and a list of group names.  *  `atlassian-group-role-actor-id` and a list of group IDs.  As a group's name can change, use of `atlassian-group-role-actor-id` is recommended. For example, `\"atlassian-group-role-actor-id\":[\"eef79f81-0b89-4fca-a736-4be531a10869\",\"77f6ab39-e755-4570-a6ae-2d7a8df0bcb8\"]`.  Add users using `atlassian-user-role-actor` and a list of account IDs. For example, `\"atlassian-user-role-actor\":[\"12345678-9abc-def1-2345-6789abcdef12\", \"abcdef12-3456-789a-bcde-f123456789ab\"]`.
    #[serde(rename = "categorisedActors", skip_serializing_if = "Option::is_none")]
    pub categorised_actors: Option<std::collections::HashMap<String, Vec<String>>>,
    /// The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl ProjectRoleActorsUpdateBean {
    pub fn new() -> ProjectRoleActorsUpdateBean {
        ProjectRoleActorsUpdateBean {
            categorised_actors: None,
            id: None,
        }
    }
}

