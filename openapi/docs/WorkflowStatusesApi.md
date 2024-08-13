# \WorkflowStatusesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_status**](WorkflowStatusesApi.md#get_status) | **GET** /rest/api/3/status/{idOrName} | Get status
[**get_statuses**](WorkflowStatusesApi.md#get_statuses) | **GET** /rest/api/3/status | Get all statuses



## get_status

> models::StatusDetails get_status(id_or_name)
Get status

Returns a status. The status must be associated with an active workflow to be returned.  If a name is used on more than one status, only the status found first is returned. Therefore, identifying the status by its ID may be preferable.  This operation can be accessed anonymously.  [Permissions](#permissions) required: None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id_or_name** | **String** | The ID or name of the status. | [required] |

### Return type

[**models::StatusDetails**](StatusDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_statuses

> Vec<models::StatusDetails> get_statuses()
Get all statuses

Returns a list of all statuses associated with active workflows.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::StatusDetails>**](StatusDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

