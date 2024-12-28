/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssuesMetaBean : Meta data describing the `issues` context variable.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuesMetaBean {
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<Box<models::IssuesJqlMetaDataBean>>,
}

impl IssuesMetaBean {
    /// Meta data describing the `issues` context variable.
    pub fn new() -> IssuesMetaBean {
        IssuesMetaBean {
            jql: None,
        }
    }
}

