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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddSecuritySchemeLevelsRequestBean {
    /// The list of scheme levels which should be added to the security scheme.
    #[serde(rename = "levels", skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<models::SecuritySchemeLevelBean>>,
}

impl AddSecuritySchemeLevelsRequestBean {
    pub fn new() -> AddSecuritySchemeLevelsRequestBean {
        AddSecuritySchemeLevelsRequestBean {
            levels: None,
        }
    }
}

