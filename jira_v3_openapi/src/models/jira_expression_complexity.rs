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

/// JiraExpressionComplexity : Details about the complexity of the analysed Jira expression.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraExpressionComplexity {
    /// Information that can be used to determine how many [expensive operations](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/#expensive-operations) the evaluation of the expression will perform. This information may be a formula or number. For example:   *  `issues.map(i => i.comments)` performs as many expensive operations as there are issues on the issues list. So this parameter returns `N`, where `N` is the size of issue list.  *  `new Issue(10010).comments` gets comments for one issue, so its complexity is `2` (`1` to retrieve issue 10010 from the database plus `1` to get its comments).
    #[serde(rename = "expensiveOperations")]
    pub expensive_operations: String,
    /// Variables used in the formula, mapped to the parts of the expression they refer to.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

impl JiraExpressionComplexity {
    /// Details about the complexity of the analysed Jira expression.
    pub fn new(expensive_operations: String) -> JiraExpressionComplexity {
        JiraExpressionComplexity {
            expensive_operations,
            variables: None,
        }
    }
}

