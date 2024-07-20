# \TasksApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_task**](TasksApi.md#cancel_task) | **POST** /rest/api/3/task/{taskId}/cancel | Cancel task
[**get_task**](TasksApi.md#get_task) | **GET** /rest/api/3/task/{taskId} | Get task



## cancel_task

> serde_json::Value cancel_task(task_id)
Cancel task

Cancels a task.  **[Permissions](#permissions) required:** either of:   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  Creator of the task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the task. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> models::TaskProgressBeanObject get_task(task_id)
Get task

Returns the status of a [long-running asynchronous task](#async).  When a task has finished, this operation returns the JSON blob applicable to the task. See the documentation of the operation that created the task for details. Task details are not permanently retained. As of September 2019, details are retained for 14 days although this period may change without notice.  **Deprecation notice:** The required OAuth 2.0 scopes will be updated on June 15, 2024.   *  `read:jira-work`  **[Permissions](#permissions) required:** either of:   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  Creator of the task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the task. | [required] |

### Return type

[**models::TaskProgressBeanObject**](TaskProgressBeanObject.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

