# \IssueSearchApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**count_issues**](IssueSearchApi.md#count_issues) | **POST** /rest/api/3/search/approximate-count | Count issues using JQL
[**get_issue_picker_resource**](IssueSearchApi.md#get_issue_picker_resource) | **GET** /rest/api/3/issue/picker | Get issue picker suggestions
[**match_issues**](IssueSearchApi.md#match_issues) | **POST** /rest/api/3/jql/match | Check issues against JQL
[**search_and_reconsile_issues_using_jql**](IssueSearchApi.md#search_and_reconsile_issues_using_jql) | **GET** /rest/api/3/search/jql | Search for issues using JQL enhanced search (GET)
[**search_and_reconsile_issues_using_jql_post**](IssueSearchApi.md#search_and_reconsile_issues_using_jql_post) | **POST** /rest/api/3/search/jql | Search for issues using JQL enhanced search (POST)
[**search_for_issues_ids**](IssueSearchApi.md#search_for_issues_ids) | **POST** /rest/api/3/search/id | Search issue IDs using JQL
[**search_for_issues_using_jql**](IssueSearchApi.md#search_for_issues_using_jql) | **GET** /rest/api/3/search | Search for issues using JQL (GET)
[**search_for_issues_using_jql_post**](IssueSearchApi.md#search_for_issues_using_jql_post) | **POST** /rest/api/3/search | Search for issues using JQL (POST)



## count_issues

> models::JqlCountResultsBean count_issues(jql_count_request_bean)
Count issues using JQL

Provide an estimated count of the issues that match the [JQL](https://confluence.atlassian.com/x/egORLQ). Recent updates might not be immediately visible in the returned output. This endpoint requires JQL to be bounded.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Issues are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jql_count_request_bean** | [**JqlCountRequestBean**](JqlCountRequestBean.md) | A JSON object containing the search request. | [required] |

### Return type

[**models::JqlCountResultsBean**](JQLCountResultsBean.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_picker_resource

> models::IssuePickerSuggestions get_issue_picker_resource(query, current_jql, current_issue_key, current_project_id, show_sub_tasks, show_sub_task_parent)
Get issue picker suggestions

Returns lists of issues matching a query string. Use this resource to provide auto-completion suggestions when the user is looking for an issue using a word or string.  This operation returns two lists:   *  `History Search` which includes issues from the user's history of created, edited, or viewed issues that contain the string in the `query` parameter.  *  `Current Search` which includes issues that match the JQL expression in `currentJQL` and contain the string in the `query` parameter.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | A string to match against text fields in the issue such as title, description, or comments. |  |
**current_jql** | Option<**String**> | A JQL query defining a list of issues to search for the query term. Note that `username` and `userkey` cannot be used as search terms for this parameter, due to privacy reasons. Use `accountId` instead. |  |
**current_issue_key** | Option<**String**> | The key of an issue to exclude from search results. For example, the issue the user is viewing when they perform this query. |  |
**current_project_id** | Option<**String**> | The ID of a project that suggested issues must belong to. |  |
**show_sub_tasks** | Option<**bool**> | Indicate whether to include subtasks in the suggestions list. |  |
**show_sub_task_parent** | Option<**bool**> | When `currentIssueKey` is a subtask, whether to include the parent issue in the suggestions if it matches the query. |  |

### Return type

[**models::IssuePickerSuggestions**](IssuePickerSuggestions.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## match_issues

> models::IssueMatches match_issues(issues_and_jql_queries)
Check issues against JQL

Checks whether one or more issues would be returned by one or more JQL queries.  **[Permissions](#permissions) required:** None, however, issues are only matched against JQL queries where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issues_and_jql_queries** | [**IssuesAndJqlQueries**](IssuesAndJqlQueries.md) |  | [required] |

### Return type

[**models::IssueMatches**](IssueMatches.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_and_reconsile_issues_using_jql

> models::SearchAndReconcileResults search_and_reconsile_issues_using_jql(search_and_reconcile_request_bean)
Search for issues using JQL enhanced search (GET)

Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ). Recent updates might not be immediately visible in the returned search results. If you need read-after-write consistency, you can utilize the `reconcileIssues` parameter to ensure stronger consistency assurances. This operation can be accessed anonymously.  If the JQL query expression is too large to be encoded as a query parameter, use the [POST](#api-rest-api-3-search-post) version of this resource.  **[Permissions](#permissions) required:** Issues are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_and_reconcile_request_bean** | [**SearchAndReconcileRequestBean**](SearchAndReconcileRequestBean.md) |  | [required] |

### Return type

[**models::SearchAndReconcileResults**](SearchAndReconcileResults.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_and_reconsile_issues_using_jql_post

> models::SearchAndReconcileResults search_and_reconsile_issues_using_jql_post(search_and_reconcile_request_bean)
Search for issues using JQL enhanced search (POST)

Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ). Recent updates might not be immediately visible in the returned search results. If you need read-after-write consistency, you can utilize the `reconcileIssues` parameter to ensure stronger consistency assurances. This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Issues are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_and_reconcile_request_bean** | [**SearchAndReconcileRequestBean**](SearchAndReconcileRequestBean.md) |  | [required] |

### Return type

[**models::SearchAndReconcileResults**](SearchAndReconcileResults.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_for_issues_ids

> models::IdSearchResults search_for_issues_ids(id_search_request_bean)
Search issue IDs using JQL

Searches for IDs of issues using [JQL](https://confluence.atlassian.com/x/egORLQ).  Use the [Search](#api-rest-api-3-search-post) endpoint if you need to fetch more than just issue IDs. The Search endpoint returns more information, but may take much longer to respond to requests. This is because it uses a different mechanism for ordering results than this endpoint and doesn't provide the total number of results for your query.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Issues are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id_search_request_bean** | [**IdSearchRequestBean**](IdSearchRequestBean.md) | A JSON object containing the search request. | [required] |

### Return type

[**models::IdSearchResults**](IdSearchResults.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_for_issues_using_jql

> models::SearchResults search_for_issues_using_jql(jql, start_at, max_results, validate_query, fields, expand, properties, fields_by_keys, fail_fast)
Search for issues using JQL (GET)

Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ).  If the JQL query expression is too large to be encoded as a query parameter, use the [POST](#api-rest-api-3-search-post) version of this resource.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Issues are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jql** | Option<**String**> | The [JQL](https://confluence.atlassian.com/x/egORLQ) that defines the search. Note:   *  If no JQL expression is provided, all issues are returned.  *  `username` and `userkey` cannot be used as search terms due to privacy reasons. Use `accountId` instead.  *  If a user has hidden their email address in their user profile, partial matches of the email address will not find the user. An exact match is required. |  |
**start_at** | Option<**i32**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. To manage page size, Jira may return fewer items per page where a large number of fields are requested. The greatest number of items returned per page is achieved when requesting `id` or `key` only. |  |[default to 50]
**validate_query** | Option<**String**> | Determines how to validate the JQL query and treat the validation results. Supported values are:   *  `strict` Returns a 400 response code if any errors are found, along with a list of all errors (and warnings).  *  `warn` Returns all errors as warnings.  *  `none` No validation is performed.  *  `true` *Deprecated* A legacy synonym for `strict`.  *  `false` *Deprecated* A legacy synonym for `warn`.  Note: If the JQL is not correctly formed a 400 response code is returned, regardless of the `validateQuery` value. |  |[default to strict]
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields to return for each issue, use it to retrieve a subset of fields. This parameter accepts a comma-separated list. Expand options include:   *  `*all` Returns all fields.  *  `*navigable` Returns navigable fields.  *  Any issue field, prefixed with a minus to exclude.  Examples:   *  `summary,comment` Returns only the summary and comments fields.  *  `-description` Returns all navigable (default) fields except description.  *  `*all,-comment` Returns all fields except comments.  This parameter may be specified multiple times. For example, `fields=field1,field2&fields=field3`.  Note: All navigable fields are returned by default. This differs from [GET issue](#api-rest-api-3-issue-issueIdOrKey-get) where the default is all fields. |  |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about issues in the response. This parameter accepts a comma-separated list. Expand options include:   *  `renderedFields` Returns field values rendered in HTML format.  *  `names` Returns the display name of each field.  *  `schema` Returns the schema describing a field type.  *  `transitions` Returns all possible transitions for the issue.  *  `operations` Returns all possible operations for the issue.  *  `editmeta` Returns information about how each field can be edited.  *  `changelog` Returns a list of recent updates to an issue, sorted by date, starting from the most recent.  *  `versionedRepresentations` Instead of `fields`, returns `versionedRepresentations` a JSON array containing each version of a field's value, with the highest numbered item representing the most recent version. |  |
**properties** | Option<[**Vec<String>**](String.md)> | A list of issue property keys for issue properties to include in the results. This parameter accepts a comma-separated list. Multiple properties can also be provided using an ampersand separated list. For example, `properties=prop1,prop2&properties=prop3`. A maximum of 5 issue property keys can be specified. |  |
**fields_by_keys** | Option<**bool**> | Reference fields by their key (rather than ID). |  |[default to false]
**fail_fast** | Option<**bool**> | Whether to fail the request quickly in case of an error while loading fields for an issue. For `failFast=true`, if one field fails, the entire operation fails. For `failFast=false`, the operation will continue even if a field fails. It will return a valid response, but without values for the failed field(s). |  |[default to false]

### Return type

[**models::SearchResults**](SearchResults.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_for_issues_using_jql_post

> models::SearchResults search_for_issues_using_jql_post(search_request_bean)
Search for issues using JQL (POST)

Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ).  There is a [GET](#api-rest-api-3-search-get) version of this resource that can be used for smaller JQL query expressions.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Issues are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_request_bean** | [**SearchRequestBean**](SearchRequestBean.md) | A JSON object containing the search request. | [required] |

### Return type

[**models::SearchResults**](SearchResults.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

