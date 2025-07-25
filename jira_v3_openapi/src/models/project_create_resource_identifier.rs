/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ProjectCreateResourceIdentifier : Every project-created entity has an ID that must be unique within the scope of the project creation. PCRI (Project Create Resource Identifier) is a standard format for creating IDs and references to other project entities. PCRI format is defined as follows: pcri:\\[entityType\\]:\\[type\\]:\\[entityId\\] entityType - the type of an entity, e.g. status, role, workflow type - PCRI type, either `id` - The ID of an entity that already exists in the target site, or `ref` - A unique reference to an entity that is being created entityId - entity identifier, if type is `id` - must be an existing entity ID that exists in the Jira site, if `ref` - must be unique across all entities in the scope of this project template creation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectCreateResourceIdentifier {
    #[serde(rename = "anID", skip_serializing_if = "Option::is_none")]
    pub an_id: Option<bool>,
    #[serde(rename = "areference", skip_serializing_if = "Option::is_none")]
    pub areference: Option<bool>,
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl ProjectCreateResourceIdentifier {
    /// Every project-created entity has an ID that must be unique within the scope of the project creation. PCRI (Project Create Resource Identifier) is a standard format for creating IDs and references to other project entities. PCRI format is defined as follows: pcri:\\[entityType\\]:\\[type\\]:\\[entityId\\] entityType - the type of an entity, e.g. status, role, workflow type - PCRI type, either `id` - The ID of an entity that already exists in the target site, or `ref` - A unique reference to an entity that is being created entityId - entity identifier, if type is `id` - must be an existing entity ID that exists in the Jira site, if `ref` - must be unique across all entities in the scope of this project template creation
    pub fn new() -> ProjectCreateResourceIdentifier {
        ProjectCreateResourceIdentifier {
            an_id: None,
            areference: None,
            entity_id: None,
            entity_type: None,
            id: None,
            r#type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "ref")]
    Ref,
}

impl Default for Type {
    fn default() -> Type {
        Self::Id
    }
}

