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

/// AvailableWorkflowSystemRule : The Atlassian provided system rules available.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableWorkflowSystemRule {
    /// The rule description.
    #[serde(rename = "description")]
    pub description: String,
    /// List of rules that conflict with this one.
    #[serde(rename = "incompatibleRuleKeys")]
    pub incompatible_rule_keys: Vec<String>,
    /// Whether the rule can be added added to an initial transition.
    #[serde(rename = "isAvailableForInitialTransition")]
    pub is_available_for_initial_transition: bool,
    /// Whether the rule is visible.
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    /// The rule name.
    #[serde(rename = "name")]
    pub name: String,
    /// The rule key.
    #[serde(rename = "ruleKey")]
    pub rule_key: String,
    /// The rule type.
    #[serde(rename = "ruleType")]
    pub rule_type: RuleType,
}

impl AvailableWorkflowSystemRule {
    /// The Atlassian provided system rules available.
    pub fn new(description: String, incompatible_rule_keys: Vec<String>, is_available_for_initial_transition: bool, is_visible: bool, name: String, rule_key: String, rule_type: RuleType) -> AvailableWorkflowSystemRule {
        AvailableWorkflowSystemRule {
            description,
            incompatible_rule_keys,
            is_available_for_initial_transition,
            is_visible,
            name,
            rule_key,
            rule_type,
        }
    }
}
/// The rule type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RuleType {
    #[serde(rename = "Condition")]
    Condition,
    #[serde(rename = "Validator")]
    Validator,
    #[serde(rename = "Function")]
    Function,
    #[serde(rename = "Screen")]
    Screen,
}

impl Default for RuleType {
    fn default() -> RuleType {
        Self::Condition
    }
}

