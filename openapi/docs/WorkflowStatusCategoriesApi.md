# \WorkflowStatusCategoriesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_status_categories**](WorkflowStatusCategoriesApi.md#get_status_categories) | **GET** /rest/api/3/statuscategory | Get all status categories
[**get_status_category**](WorkflowStatusCategoriesApi.md#get_status_category) | **GET** /rest/api/3/statuscategory/{idOrKey} | Get status category



## get_status_categories

> Vec<models::StatusCategory> get_status_categories()
Get all status categories

Returns a list of all status categories.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::StatusCategory>**](StatusCategory.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status_category

> models::StatusCategory get_status_category(id_or_key)
Get status category

Returns a status category. Status categories provided a mechanism for categorizing [statuses](#api-rest-api-3-status-idOrName-get).  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id_or_key** | **String** | The ID or key of the status category. | [required] |

### Return type

[**models::StatusCategory**](StatusCategory.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

