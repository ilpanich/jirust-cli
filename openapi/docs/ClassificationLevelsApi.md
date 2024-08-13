# \ClassificationLevelsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_user_data_classification_levels**](ClassificationLevelsApi.md#get_all_user_data_classification_levels) | **GET** /rest/api/3/classification-levels | Get all classification levels



## get_all_user_data_classification_levels

> models::DataClassificationLevelsBean get_all_user_data_classification_levels(status, order_by)
Get all classification levels

Returns all classification levels.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**Vec<String>**](String.md)> | Optional set of statuses to filter by. |  |
**order_by** | Option<**String**> | Ordering of the results by a given field. If not provided, values will not be sorted. |  |

### Return type

[**models::DataClassificationLevelsBean**](DataClassificationLevelsBean.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

