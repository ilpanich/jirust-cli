/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`get_bulk_editable_fields`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBulkEditableFieldsError {
    Status400(models::BulkOperationErrorResponse),
    Status401(models::BulkOperationErrorResponse),
    Status403(models::BulkOperationErrorResponse),
    Status404(models::BulkOperationErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_bulk_operation_progress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBulkOperationProgressError {
    Status400(models::BulkOperationErrorResponse),
    Status401(models::BulkOperationErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`submit_bulk_edit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitBulkEditError {
    Status400(models::BulkOperationErrorResponse),
    Status401(models::BulkOperationErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`submit_bulk_move`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitBulkMoveError {
    Status400(models::BulkOperationErrorResponse),
    Status401(models::BulkOperationErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Use this API to get a list of fields visible to the user to perform bulk edit operations. You can pass single or multiple issues in the query to get eligible editable fields. This API uses pagination to return responses, delivering 50 fields at a time.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  Depending on the field, any field-specific permissions required to edit it.
pub async fn get_bulk_editable_fields(configuration: &configuration::Configuration, issue_ids_or_keys: &str, search_text: Option<&str>, ending_before: Option<&str>, starting_after: Option<&str>) -> Result<models::BulkEditGetFields, Error<GetBulkEditableFieldsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/bulk/issues/fields", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("issueIdsOrKeys", &issue_ids_or_keys.to_string())]);
    if let Some(ref local_var_str) = search_text {
        local_var_req_builder = local_var_req_builder.query(&[("searchText", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ending_before {
        local_var_req_builder = local_var_req_builder.query(&[("endingBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = starting_after {
        local_var_req_builder = local_var_req_builder.query(&[("startingAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetBulkEditableFieldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Use this to get the progress state for the specified bulk operation `taskId`.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Administer Jira [global permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/), or be the creator of the task.  If the task is running, this resource will return:      {\"taskId\":\"10779\",\"status\":\"RUNNING\",\"progressPercent\":65,\"submittedBy\":{\"accountId\":\"5b10a2844c20165700ede21g\"},\"created\":1690180055963,\"started\":1690180056206,\"updated\":169018005829}  If the task has completed, then this resource will return:      {\"processedAccessibleIssues\":[10001,10002],\"created\":1709189449954,\"progressPercent\":100,\"started\":1709189450154,\"status\":\"COMPLETE\",\"submittedBy\":{\"accountId\":\"5b10a2844c20165700ede21g\"},\"invalidOrInaccessibleIssueCount\":0,\"taskId\":\"10000\",\"totalIssueCount\":2,\"updated\":1709189450354}  **Note:** You can view task progress for up to 14 days from creation.
pub async fn get_bulk_operation_progress(configuration: &configuration::Configuration, task_id: &str) -> Result<models::BulkOperationProgress, Error<GetBulkOperationProgressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/bulk/queue/{taskId}", local_var_configuration.base_path, taskId=crate::apis::urlencode(task_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetBulkOperationProgressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Use this API to submit a bulk edit request and simultaneously edit multiple issues. There are limits applied to the number of issues and fields that can be edited. A single request can accommodate a maximum of 1000 issues (including subtasks) and 200 fields.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.  *  Edit [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn submit_bulk_edit(configuration: &configuration::Configuration, issue_bulk_edit_payload: models::IssueBulkEditPayload) -> Result<models::SubmittedBulkOperation, Error<SubmitBulkEditError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/bulk/issues/fields", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&issue_bulk_edit_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SubmitBulkEditError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Use this API to submit a bulk issue move request. You can move multiple issues, but they must all be moved to and from a single project, issue type, and parent. You can't move more than 1000 issues (including subtasks) at once.  #### Scenarios: ####  This is an early version of the API and it doesn't have full feature parity with the Bulk Move UI experience.   *  Moving issue of type A to issue of type B in the same project or a different project: `SUPPORTED`  *  Moving multiple issues of type A in one project to multiple issues of type B in the same project or a different project: **`SUPPORTED`**  *  Moving a standard parent issue of type A with its multiple subtask issue types in one project to standard issue of type B and multiple subtask issue types in the same project or a different project: `SUPPORTED`  *  Moving an epic issue with its child issues to a different project without losing their relation: `NOT SUPPORTED`       (Workaround: Move them individually and stitch the relationship back with the Bulk Edit API)  #### Limits applied to bulk issue moves: ####  When using the bulk move, keep in mind that there are limits on the number of issues and fields you can include.   *  You can move up to 1,000 issues in a single operation, including any subtasks.  *  All issues must originate from the same project and share the same issue type and parent.  *  The total combined number of fields across all issues must not exceed 1,500,000. For example, if each issue includes 15,000 fields, then the maximum number of issues that can be moved is 100.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Move [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in source projects.  *  Create [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in destination projects.  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in destination projects, if moving subtasks only.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn submit_bulk_move(configuration: &configuration::Configuration, issue_bulk_move_payload: models::IssueBulkMovePayload) -> Result<models::SubmittedBulkOperation, Error<SubmitBulkMoveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/bulk/issues/move", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&issue_bulk_move_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SubmitBulkMoveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

