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

/// WorkflowTransitions : The transitions of the workflow. Note that a transition can have either the deprecated `to`/`from` fields or the `toStatusReference`/`links` fields, but never both nor a combination.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitions {
    /// The post-functions of the transition.
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<models::WorkflowRuleConfiguration>>,
    #[serde(rename = "conditions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Option<Box<models::ConditionGroupConfiguration>>>,
    /// The custom event ID of the transition.
    #[serde(rename = "customIssueEventId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_issue_event_id: Option<Option<String>>,
    /// The description of the transition.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The statuses and ports that the transition can start from. This field is deprecated - use `toStatusReference`/`links` instead.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<models::WorkflowStatusAndPort>>,
    /// The ID of the transition.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The statuses the transition can start from, and the mapping of ports between the statuses.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<models::WorkflowTransitionLinks>>,
    /// The name of the transition.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The properties of the transition.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to: Option<Option<Box<models::WorkflowStatusAndPort>>>,
    /// The status the transition goes to.
    #[serde(rename = "toStatusReference", skip_serializing_if = "Option::is_none")]
    pub to_status_reference: Option<String>,
    #[serde(rename = "transitionScreen", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transition_screen: Option<Option<Box<models::WorkflowRuleConfiguration>>>,
    /// The triggers of the transition.
    #[serde(rename = "triggers", skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<models::WorkflowTrigger>>,
    /// The transition type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The validators of the transition.
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<models::WorkflowRuleConfiguration>>,
}

impl WorkflowTransitions {
    /// The transitions of the workflow. Note that a transition can have either the deprecated `to`/`from` fields or the `toStatusReference`/`links` fields, but never both nor a combination.
    pub fn new() -> WorkflowTransitions {
        WorkflowTransitions {
            actions: None,
            conditions: None,
            custom_issue_event_id: None,
            description: None,
            from: None,
            id: None,
            links: None,
            name: None,
            properties: None,
            to: None,
            to_status_reference: None,
            transition_screen: None,
            triggers: None,
            r#type: None,
            validators: None,
        }
    }
}
/// The transition type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "INITIAL")]
    Initial,
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "DIRECTED")]
    Directed,
}

impl Default for Type {
    fn default() -> Type {
        Self::Initial
    }
}

