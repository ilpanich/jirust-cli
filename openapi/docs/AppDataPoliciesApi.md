# \AppDataPoliciesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_policies**](AppDataPoliciesApi.md#get_policies) | **GET** /rest/api/3/data-policy/project | Get data policy for projects
[**get_policy**](AppDataPoliciesApi.md#get_policy) | **GET** /rest/api/3/data-policy | Get data policy for the workspace



## get_policies

> models::ProjectDataPolicies get_policies(ids)
Get data policy for projects

Returns data policies for the projects specified in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<**String**> | A list of project identifiers. This parameter accepts a comma-separated list. |  |

### Return type

[**models::ProjectDataPolicies**](ProjectDataPolicies.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy

> models::WorkspaceDataPolicy get_policy()
Get data policy for the workspace

Returns data policy for the workspace.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WorkspaceDataPolicy**](WorkspaceDataPolicy.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

