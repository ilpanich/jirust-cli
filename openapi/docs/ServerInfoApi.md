# \ServerInfoApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_server_info**](ServerInfoApi.md#get_server_info) | **GET** /rest/api/3/serverInfo | Get Jira instance info



## get_server_info

> models::ServerInformation get_server_info()
Get Jira instance info

Returns information about the Jira instance.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerInformation**](ServerInformation.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

