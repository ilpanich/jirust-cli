/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`get_issue_picker_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIssuePickerResourceError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`match_issues`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchIssuesError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_for_issues_ids`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchForIssuesIdsError {
    Status400(),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_for_issues_using_jql`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchForIssuesUsingJqlError {
    Status400(),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_for_issues_using_jql_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchForIssuesUsingJqlPostError {
    Status400(),
    Status401(),
    UnknownValue(serde_json::Value),
}


/// Returns lists of issues matching a query string. Use this resource to provide auto-completion suggestions when the user is looking for an issue using a word or string.  This operation returns two lists:   *  `History Search` which includes issues from the user's history of created, edited, or viewed issues that contain the string in the `query` parameter.  *  `Current Search` which includes issues that match the JQL expression in `currentJQL` and contain the string in the `query` parameter.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.
pub async fn get_issue_picker_resource(configuration: &configuration::Configuration, query: Option<&str>, current_jql: Option<&str>, current_issue_key: Option<&str>, current_project_id: Option<&str>, show_sub_tasks: Option<bool>, show_sub_task_parent: Option<bool>) -> Result<models::IssuePickerSuggestions, Error<GetIssuePickerResourceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/issue/picker", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = query {
        local_var_req_builder = local_var_req_builder.query(&[("query", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = current_jql {
        local_var_req_builder = local_var_req_builder.query(&[("currentJQL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = current_issue_key {
        local_var_req_builder = local_var_req_builder.query(&[("currentIssueKey", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = current_project_id {
        local_var_req_builder = local_var_req_builder.query(&[("currentProjectId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = show_sub_tasks {
        local_var_req_builder = local_var_req_builder.query(&[("showSubTasks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = show_sub_task_parent {
        local_var_req_builder = local_var_req_builder.query(&[("showSubTaskParent", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetIssuePickerResourceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Checks whether one or more issues would be returned by one or more JQL queries.  **[Permissions](#permissions) required:** None, however, issues are only matched against JQL queries where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn match_issues(configuration: &configuration::Configuration, issues_and_jql_queries: models::IssuesAndJqlQueries) -> Result<models::IssueMatches, Error<MatchIssuesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/jql/match", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&issues_and_jql_queries);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MatchIssuesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Searches for IDs of issues using [JQL](https://confluence.atlassian.com/x/egORLQ).  Use the [Search](#api-rest-api-3-search-post) endpoint if you need to fetch more than just issue IDs. The Search endpoint returns more information, but may take much longer to respond to requests. This is because it uses a different mechanism for ordering results than this endpoint and doesn't provide the total number of results for your query.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Issues are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn search_for_issues_ids(configuration: &configuration::Configuration, id_search_request_bean: models::IdSearchRequestBean) -> Result<models::IdSearchResults, Error<SearchForIssuesIdsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/search/id", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&id_search_request_bean);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchForIssuesIdsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ).  If the JQL query expression is too large to be encoded as a query parameter, use the [POST](#api-rest-api-3-search-post) version of this resource.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Issues are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn search_for_issues_using_jql(configuration: &configuration::Configuration, jql: Option<&str>, start_at: Option<i32>, max_results: Option<i32>, validate_query: Option<&str>, fields: Option<Vec<String>>, expand: Option<&str>, properties: Option<Vec<String>>, fields_by_keys: Option<bool>, fail_fast: Option<bool>) -> Result<models::SearchResults, Error<SearchForIssuesUsingJqlError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/search", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = jql {
        local_var_req_builder = local_var_req_builder.query(&[("jql", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_at {
        local_var_req_builder = local_var_req_builder.query(&[("startAt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_results {
        local_var_req_builder = local_var_req_builder.query(&[("maxResults", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = validate_query {
        local_var_req_builder = local_var_req_builder.query(&[("validateQuery", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = properties {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("properties".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("properties", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = fields_by_keys {
        local_var_req_builder = local_var_req_builder.query(&[("fieldsByKeys", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fail_fast {
        local_var_req_builder = local_var_req_builder.query(&[("failFast", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SearchForIssuesUsingJqlError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ).  There is a [GET](#api-rest-api-3-search-get) version of this resource that can be used for smaller JQL query expressions.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Issues are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn search_for_issues_using_jql_post(configuration: &configuration::Configuration, search_request_bean: models::SearchRequestBean) -> Result<models::SearchResults, Error<SearchForIssuesUsingJqlPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/search", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&search_request_bean);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchForIssuesUsingJqlPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

