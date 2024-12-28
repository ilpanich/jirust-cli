# \JqlFunctionsAppsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_precomputations**](JqlFunctionsAppsApi.md#get_precomputations) | **GET** /rest/api/3/jql/function/computation | Get precomputations (apps)
[**get_precomputations_by_id**](JqlFunctionsAppsApi.md#get_precomputations_by_id) | **POST** /rest/api/3/jql/function/computation/search | Get precomputations by ID (apps)
[**update_precomputations**](JqlFunctionsAppsApi.md#update_precomputations) | **POST** /rest/api/3/jql/function/computation | Update precomputations (apps)



## get_precomputations

> models::PageBean2JqlFunctionPrecomputationBean get_precomputations(function_key, start_at, max_results, order_by)
Get precomputations (apps)

Returns the list of a function's precomputations along with information about when they were created, updated, and last used. Each precomputation has a `value` \\- the JQL fragment to replace the custom function clause with.  **[Permissions](#permissions) required:** This API is only accessible to apps and apps can only inspect their own functions.  The new `read:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_key** | Option<[**Vec<String>**](String.md)> | The function key in format:   *  Forge: `ari:cloud:ecosystem::extension/[App ID]/[Environment ID]/static/[Function key from manifest]`  *  Connect: `[App key]__[Module key]` |  |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `functionKey` Sorts by the functionKey.  *  `used` Sorts by the used timestamp.  *  `created` Sorts by the created timestamp.  *  `updated` Sorts by the updated timestamp. |  |

### Return type

[**models::PageBean2JqlFunctionPrecomputationBean**](PageBean2JqlFunctionPrecomputationBean.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_precomputations_by_id

> models::JqlFunctionPrecomputationGetByIdResponse get_precomputations_by_id(jql_function_precomputation_get_by_id_request, order_by)
Get precomputations by ID (apps)

Returns function precomputations by IDs, along with information about when they were created, updated, and last used. Each precomputation has a `value` \\- the JQL fragment to replace the custom function clause with.  **[Permissions](#permissions) required:** This API is only accessible to apps and apps can only inspect their own functions.  The new `read:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jql_function_precomputation_get_by_id_request** | [**JqlFunctionPrecomputationGetByIdRequest**](JqlFunctionPrecomputationGetByIdRequest.md) |  | [required] |
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `functionKey` Sorts by the functionKey.  *  `used` Sorts by the used timestamp.  *  `created` Sorts by the created timestamp.  *  `updated` Sorts by the updated timestamp. |  |

### Return type

[**models::JqlFunctionPrecomputationGetByIdResponse**](JqlFunctionPrecomputationGetByIdResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_precomputations

> models::JqlFunctionPrecomputationUpdateResponse update_precomputations(jql_function_precomputation_update_request_bean, skip_not_found_precomputations)
Update precomputations (apps)

Update the precomputation value of a function created by a Forge/Connect app.  **[Permissions](#permissions) required:** An API for apps to update their own precomputations.  The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jql_function_precomputation_update_request_bean** | [**JqlFunctionPrecomputationUpdateRequestBean**](JqlFunctionPrecomputationUpdateRequestBean.md) |  | [required] |
**skip_not_found_precomputations** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::JqlFunctionPrecomputationUpdateResponse**](JqlFunctionPrecomputationUpdateResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

