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

/// JqlQuery : A parsed JQL query.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQuery {
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Box<models::JqlQueryOrderByClause>>,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub r#where: Option<Box<models::JqlQueryClause>>,
}

impl JqlQuery {
    /// A parsed JQL query.
    pub fn new() -> JqlQuery {
        JqlQuery {
            order_by: None,
            r#where: None,
        }
    }
}

