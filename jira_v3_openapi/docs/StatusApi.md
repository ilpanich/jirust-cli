# \StatusApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_statuses**](StatusApi.md#create_statuses) | **POST** /rest/api/3/statuses | Bulk create statuses
[**delete_statuses_by_id**](StatusApi.md#delete_statuses_by_id) | **DELETE** /rest/api/3/statuses | Bulk delete Statuses
[**get_project_issue_type_usages_for_status**](StatusApi.md#get_project_issue_type_usages_for_status) | **GET** /rest/api/3/statuses/{statusId}/project/{projectId}/issueTypeUsages | Get issue type usages by status and project
[**get_project_usages_for_status**](StatusApi.md#get_project_usages_for_status) | **GET** /rest/api/3/statuses/{statusId}/projectUsages | Get project usages by status
[**get_statuses_by_id**](StatusApi.md#get_statuses_by_id) | **GET** /rest/api/3/statuses | Bulk get statuses
[**get_workflow_usages_for_status**](StatusApi.md#get_workflow_usages_for_status) | **GET** /rest/api/3/statuses/{statusId}/workflowUsages | Get workflow usages by status
[**search**](StatusApi.md#search) | **GET** /rest/api/3/statuses/search | Search statuses paginated
[**update_statuses**](StatusApi.md#update_statuses) | **PUT** /rest/api/3/statuses | Bulk update statuses



## create_statuses

> Vec<models::JiraStatus> create_statuses(status_create_request)
Bulk create statuses

Creates statuses for a global or project scope.  **[Permissions](#permissions) required:**   *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_create_request** | [**StatusCreateRequest**](StatusCreateRequest.md) | Details of the statuses being created and their scope. | [required] |

### Return type

[**Vec<models::JiraStatus>**](JiraStatus.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_statuses_by_id

> serde_json::Value delete_statuses_by_id(id)
Bulk delete Statuses

Deletes statuses by ID.  **[Permissions](#permissions) required:**   *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<String>**](String.md) | The list of status IDs. To include multiple IDs, provide an ampersand-separated list. For example, id=10000&id=10001.  Min items `1`, Max items `50` | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_issue_type_usages_for_status

> models::StatusProjectIssueTypeUsageDto get_project_issue_type_usages_for_status(status_id, project_id, next_page_token, max_results)
Get issue type usages by status and project

Returns a page of issue types in a project using a given status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_id** | **String** | The statusId to fetch issue type usages for | [required] |
**project_id** | **String** | The projectId to fetch issue type usages for | [required] |
**next_page_token** | Option<**String**> | The cursor for pagination |  |
**max_results** | Option<**i32**> | The maximum number of results to return. Must be an integer between 1 and 200. |  |[default to 50]

### Return type

[**models::StatusProjectIssueTypeUsageDto**](StatusProjectIssueTypeUsageDTO.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_usages_for_status

> models::StatusProjectUsageDto get_project_usages_for_status(status_id, next_page_token, max_results)
Get project usages by status

Returns a page of projects using a given status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_id** | **String** | The statusId to fetch project usages for | [required] |
**next_page_token** | Option<**String**> | The cursor for pagination |  |
**max_results** | Option<**i32**> | The maximum number of results to return. Must be an integer between 1 and 200. |  |[default to 50]

### Return type

[**models::StatusProjectUsageDto**](StatusProjectUsageDTO.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_statuses_by_id

> Vec<models::JiraStatus> get_statuses_by_id(id, expand)
Bulk get statuses

Returns a list of the statuses specified by one or more status IDs.  **[Permissions](#permissions) required:**   *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<String>**](String.md) | The list of status IDs. To include multiple IDs, provide an ampersand-separated list. For example, id=10000&id=10001.  Min items `1`, Max items `50` | [required] |
**expand** | Option<**String**> | Deprecated. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-2298) for details.  Use [expand](#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:   *  `usages` Returns the project and issue types that use the status in their workflow.  *  `workflowUsages` Returns the workflows that use the status. |  |

### Return type

[**Vec<models::JiraStatus>**](JiraStatus.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_usages_for_status

> models::StatusWorkflowUsageDto get_workflow_usages_for_status(status_id, next_page_token, max_results)
Get workflow usages by status

Returns a page of workflows using a given status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_id** | **String** | The statusId to fetch workflow usages for | [required] |
**next_page_token** | Option<**String**> | The cursor for pagination |  |
**max_results** | Option<**i32**> | The maximum number of results to return. Must be an integer between 1 and 200. |  |[default to 50]

### Return type

[**models::StatusWorkflowUsageDto**](StatusWorkflowUsageDTO.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search

> models::PageOfStatuses search(expand, project_id, start_at, max_results, search_string, status_category)
Search statuses paginated

Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro/#pagination) list of statuses that match a search on name or project.  **[Permissions](#permissions) required:**   *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | Option<**String**> | Deprecated. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-2298) for details.  Use [expand](#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:   *  `usages` Returns the project and issue types that use the status in their workflow.  *  `workflowUsages` Returns the workflows that use the status. |  |
**project_id** | Option<**String**> | The project the status is part of or null for global statuses. |  |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 200]
**search_string** | Option<**String**> | Term to match status names against or null to search for all statuses in the search scope. |  |
**status_category** | Option<**String**> | Category of the status to filter by. The supported values are: `TODO`, `IN_PROGRESS`, and `DONE`. |  |

### Return type

[**models::PageOfStatuses**](PageOfStatuses.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_statuses

> serde_json::Value update_statuses(status_update_request)
Bulk update statuses

Updates statuses by ID.  **[Permissions](#permissions) required:**   *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_update_request** | [**StatusUpdateRequest**](StatusUpdateRequest.md) | The list of statuses that will be updated. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

