/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CreateWorkflowTransitionRulesDetails : The details of a workflow transition rules.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWorkflowTransitionRulesDetails {
    /// The workflow conditions.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Box<models::CreateWorkflowCondition>>,
    /// The workflow post functions.  **Note:** The default post functions are always added to the *initial* transition, as in:      \"postFunctions\": [         {             \"type\": \"IssueCreateFunction\"         },         {             \"type\": \"IssueReindexFunction\"         },         {             \"type\": \"FireIssueEventFunction\",             \"configuration\": {                 \"event\": {                     \"id\": \"1\",                     \"name\": \"issue_created\"                 }             }         }     ]  **Note:** The default post functions are always added to the *global* and *directed* transitions, as in:      \"postFunctions\": [         {             \"type\": \"UpdateIssueStatusFunction\"         },         {             \"type\": \"CreateCommentFunction\"         },         {             \"type\": \"GenerateChangeHistoryFunction\"         },         {             \"type\": \"IssueReindexFunction\"         },         {             \"type\": \"FireIssueEventFunction\",             \"configuration\": {                 \"event\": {                     \"id\": \"13\",                     \"name\": \"issue_generic\"                 }             }         }     ]
    #[serde(rename = "postFunctions", skip_serializing_if = "Option::is_none")]
    pub post_functions: Option<Vec<models::CreateWorkflowTransitionRule>>,
    /// The workflow validators.  **Note:** The default permission validator is always added to the *initial* transition, as in:      \"validators\": [         {             \"type\": \"PermissionValidator\",             \"configuration\": {                 \"permissionKey\": \"CREATE_ISSUES\"             }         }     ]
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<models::CreateWorkflowTransitionRule>>,
}

impl CreateWorkflowTransitionRulesDetails {
    /// The details of a workflow transition rules.
    pub fn new() -> CreateWorkflowTransitionRulesDetails {
        CreateWorkflowTransitionRulesDetails {
            conditions: None,
            post_functions: None,
            validators: None,
        }
    }
}

