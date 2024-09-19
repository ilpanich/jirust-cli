# \IssuePrioritiesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_priority**](IssuePrioritiesApi.md#create_priority) | **POST** /rest/api/3/priority | Create priority
[**delete_priority**](IssuePrioritiesApi.md#delete_priority) | **DELETE** /rest/api/3/priority/{id} | Delete priority
[**get_priorities**](IssuePrioritiesApi.md#get_priorities) | **GET** /rest/api/3/priority | Get priorities
[**get_priority**](IssuePrioritiesApi.md#get_priority) | **GET** /rest/api/3/priority/{id} | Get priority
[**move_priorities**](IssuePrioritiesApi.md#move_priorities) | **PUT** /rest/api/3/priority/move | Move priorities
[**search_priorities**](IssuePrioritiesApi.md#search_priorities) | **GET** /rest/api/3/priority/search | Search priorities
[**set_default_priority**](IssuePrioritiesApi.md#set_default_priority) | **PUT** /rest/api/3/priority/default | Set default priority
[**update_priority**](IssuePrioritiesApi.md#update_priority) | **PUT** /rest/api/3/priority/{id} | Update priority



## create_priority

> models::PriorityId create_priority(create_priority_details)
Create priority

Creates an issue priority.  Deprecation applies to iconUrl param in request body which will be sunset on 16th Mar 2025. For more details refer to [changelog](https://developer.atlassian.com/changelog/#CHANGE-1525).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_priority_details** | [**CreatePriorityDetails**](CreatePriorityDetails.md) |  | [required] |

### Return type

[**models::PriorityId**](PriorityId.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_priority

> delete_priority(id)
Delete priority

Deletes an issue priority.  This operation is [asynchronous](#async). Follow the `location` link in the response to determine the status of the task and use [Get task](#api-rest-api-3-task-taskId-get) to obtain subsequent updates.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue priority. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_priorities

> Vec<models::Priority> get_priorities()
Get priorities

Returns the list of all issue priorities.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Priority>**](Priority.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_priority

> models::Priority get_priority(id)
Get priority

Returns an issue priority.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue priority. | [required] |

### Return type

[**models::Priority**](Priority.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_priorities

> serde_json::Value move_priorities(reorder_issue_priorities)
Move priorities

Changes the order of issue priorities.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reorder_issue_priorities** | [**ReorderIssuePriorities**](ReorderIssuePriorities.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_priorities

> models::PageBeanPriority search_priorities(start_at, max_results, id, project_id, priority_name, only_default, expand)
Search priorities

Returns a [paginated](#pagination) list of priorities. The list can contain all priorities or a subset determined by any combination of these criteria:   *  a list of priority IDs. Any invalid priority IDs are ignored.  *  a list of project IDs. Only priorities that are available in these projects will be returned. Any invalid project IDs are ignored.  *  whether the field configuration is a default. This returns priorities from company-managed (classic) projects only, as there is no concept of default priorities in team-managed projects.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<String>**](String.md)> | The list of priority IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=2&id=3`. |  |
**project_id** | Option<[**Vec<String>**](String.md)> | The list of projects IDs. To include multiple IDs, provide an ampersand-separated list. For example, `projectId=10010&projectId=10111`. |  |
**priority_name** | Option<**String**> | The name of priority to search for. |  |[default to ]
**only_default** | Option<**bool**> | Whether only the default priority is returned. |  |[default to false]
**expand** | Option<**String**> | Use `schemes` to return the associated priority schemes for each priority. Limited to returning first 15 priority schemes per priority. |  |[default to ]

### Return type

[**models::PageBeanPriority**](PageBeanPriority.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_priority

> serde_json::Value set_default_priority(set_default_priority_request)
Set default priority

Sets default issue priority.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_default_priority_request** | [**SetDefaultPriorityRequest**](SetDefaultPriorityRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_priority

> serde_json::Value update_priority(id, update_priority_details)
Update priority

Updates an issue priority.  At least one request body parameter must be defined.  Deprecation applies to iconUrl param in request body which will be sunset on 16th Mar 2025. For more details refer to [changelog](https://developer.atlassian.com/changelog/#CHANGE-1525).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue priority. | [required] |
**update_priority_details** | [**UpdatePriorityDetails**](UpdatePriorityDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

