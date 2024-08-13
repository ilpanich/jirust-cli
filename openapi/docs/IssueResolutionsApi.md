# \IssueResolutionsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_resolution**](IssueResolutionsApi.md#create_resolution) | **POST** /rest/api/3/resolution | Create resolution
[**delete_resolution**](IssueResolutionsApi.md#delete_resolution) | **DELETE** /rest/api/3/resolution/{id} | Delete resolution
[**get_resolution**](IssueResolutionsApi.md#get_resolution) | **GET** /rest/api/3/resolution/{id} | Get resolution
[**get_resolutions**](IssueResolutionsApi.md#get_resolutions) | **GET** /rest/api/3/resolution | Get resolutions
[**move_resolutions**](IssueResolutionsApi.md#move_resolutions) | **PUT** /rest/api/3/resolution/move | Move resolutions
[**search_resolutions**](IssueResolutionsApi.md#search_resolutions) | **GET** /rest/api/3/resolution/search | Search resolutions
[**set_default_resolution**](IssueResolutionsApi.md#set_default_resolution) | **PUT** /rest/api/3/resolution/default | Set default resolution
[**update_resolution**](IssueResolutionsApi.md#update_resolution) | **PUT** /rest/api/3/resolution/{id} | Update resolution



## create_resolution

> models::ResolutionId create_resolution(create_resolution_details)
Create resolution

Creates an issue resolution.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_resolution_details** | [**CreateResolutionDetails**](CreateResolutionDetails.md) |  | [required] |

### Return type

[**models::ResolutionId**](ResolutionId.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resolution

> delete_resolution(id, replace_with)
Delete resolution

Deletes an issue resolution.  This operation is [asynchronous](#async). Follow the `location` link in the response to determine the status of the task and use [Get task](#api-rest-api-3-task-taskId-get) to obtain subsequent updates.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue resolution. | [required] |
**replace_with** | **String** | The ID of the issue resolution that will replace the currently selected resolution. | [required] |[default to ]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resolution

> models::Resolution get_resolution(id)
Get resolution

Returns an issue resolution value.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue resolution value. | [required] |

### Return type

[**models::Resolution**](Resolution.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resolutions

> Vec<models::Resolution> get_resolutions()
Get resolutions

Returns a list of all issue resolution values.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Resolution>**](Resolution.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_resolutions

> serde_json::Value move_resolutions(reorder_issue_resolutions_request)
Move resolutions

Changes the order of issue resolutions.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reorder_issue_resolutions_request** | [**ReorderIssueResolutionsRequest**](ReorderIssueResolutionsRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_resolutions

> models::PageBeanResolutionJsonBean search_resolutions(start_at, max_results, id, only_default)
Search resolutions

Returns a [paginated](#pagination) list of resolutions. The list can contain all resolutions or a subset determined by any combination of these criteria:   *  a list of resolutions IDs.  *  whether the field configuration is a default. This returns resolutions from company-managed (classic) projects only, as there is no concept of default resolutions in team-managed projects.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<String>**](String.md)> | The list of resolutions IDs to be filtered out |  |
**only_default** | Option<**bool**> | When set to true, return default only, when IDs provided, if none of them is default, return empty page. Default value is false |  |[default to false]

### Return type

[**models::PageBeanResolutionJsonBean**](PageBeanResolutionJsonBean.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_resolution

> serde_json::Value set_default_resolution(set_default_resolution_request)
Set default resolution

Sets default issue resolution.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_default_resolution_request** | [**SetDefaultResolutionRequest**](SetDefaultResolutionRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_resolution

> serde_json::Value update_resolution(id, update_resolution_details)
Update resolution

Updates an issue resolution.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue resolution. | [required] |
**update_resolution_details** | [**UpdateResolutionDetails**](UpdateResolutionDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

