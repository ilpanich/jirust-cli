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

/// JqlQueryOrderByClauseElement : An element of the order-by JQL clause.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryOrderByClauseElement {
    /// The direction in which to order the results.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(rename = "field")]
    pub field: Box<models::JqlQueryField>,
}

impl JqlQueryOrderByClauseElement {
    /// An element of the order-by JQL clause.
    pub fn new(field: models::JqlQueryField) -> JqlQueryOrderByClauseElement {
        JqlQueryOrderByClauseElement {
            direction: None,
            field: Box::new(field),
        }
    }
}
/// The direction in which to order the results.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Asc
    }
}

