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
pub struct JiraExpressionsComplexityBean {
    /// The number of Jira REST API beans returned in the response.
    #[serde(rename = "beans")]
    pub beans: Box<models::JiraExpressionsComplexityValueBean>,
    /// The number of expensive operations executed while evaluating the expression. Expensive operations are those that load additional data, such as entity properties, comments, or custom fields.
    #[serde(rename = "expensiveOperations")]
    pub expensive_operations: Box<models::JiraExpressionsComplexityValueBean>,
    /// The number of primitive values returned in the response.
    #[serde(rename = "primitiveValues")]
    pub primitive_values: Box<models::JiraExpressionsComplexityValueBean>,
    /// The number of steps it took to evaluate the expression, where a step is a high-level operation performed by the expression. A step is an operation such as arithmetic, accessing a property, accessing a context variable, or calling a function.
    #[serde(rename = "steps")]
    pub steps: Box<models::JiraExpressionsComplexityValueBean>,
}

impl JiraExpressionsComplexityBean {
    pub fn new(beans: models::JiraExpressionsComplexityValueBean, expensive_operations: models::JiraExpressionsComplexityValueBean, primitive_values: models::JiraExpressionsComplexityValueBean, steps: models::JiraExpressionsComplexityValueBean) -> JiraExpressionsComplexityBean {
        JiraExpressionsComplexityBean {
            beans: Box::new(beans),
            expensive_operations: Box::new(expensive_operations),
            primitive_values: Box::new(primitive_values),
            steps: Box::new(steps),
        }
    }
}

