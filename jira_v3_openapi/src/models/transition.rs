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

/// Transition : Details of a workflow transition.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    /// The description of the transition.
    #[serde(rename = "description")]
    pub description: String,
    /// The statuses the transition can start from.
    #[serde(rename = "from")]
    pub from: Vec<String>,
    /// The ID of the transition.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the transition.
    #[serde(rename = "name")]
    pub name: String,
    /// The properties of the transition.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Box<models::WorkflowRules>>,
    #[serde(rename = "screen", skip_serializing_if = "Option::is_none")]
    pub screen: Option<Box<models::TransitionScreenDetails>>,
    /// The status the transition goes to.
    #[serde(rename = "to")]
    pub to: String,
    /// The type of the transition.
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl Transition {
    /// Details of a workflow transition.
    pub fn new(description: String, from: Vec<String>, id: String, name: String, to: String, r#type: Type) -> Transition {
        Transition {
            description,
            from,
            id,
            name,
            properties: None,
            rules: None,
            screen: None,
            to,
            r#type,
        }
    }
}
/// The type of the transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "initial")]
    Initial,
    #[serde(rename = "directed")]
    Directed,
}

impl Default for Type {
    fn default() -> Type {
        Self::Global
    }
}

