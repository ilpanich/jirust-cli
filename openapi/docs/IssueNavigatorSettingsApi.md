# \IssueNavigatorSettingsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_issue_navigator_default_columns**](IssueNavigatorSettingsApi.md#get_issue_navigator_default_columns) | **GET** /rest/api/3/settings/columns | Get issue navigator default columns
[**set_issue_navigator_default_columns**](IssueNavigatorSettingsApi.md#set_issue_navigator_default_columns) | **PUT** /rest/api/3/settings/columns | Set issue navigator default columns



## get_issue_navigator_default_columns

> Vec<models::ColumnItem> get_issue_navigator_default_columns()
Get issue navigator default columns

Returns the default issue navigator columns.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ColumnItem>**](ColumnItem.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_issue_navigator_default_columns

> set_issue_navigator_default_columns(column_request_body)
Set issue navigator default columns

Sets the default issue navigator columns.  The `columns` parameter accepts a navigable field value and is expressed as HTML form data. To specify multiple columns, pass multiple `columns` parameters. For example, in curl:  `curl -X PUT -d columns=summary -d columns=description https://your-domain.atlassian.net/rest/api/3/settings/columns`  If no column details are sent, then all default columns are removed.  A navigable field is one that can be used as a column on the issue navigator. Find details of navigable issue columns using [Get fields](#api-rest-api-3-field-get).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**column_request_body** | [**ColumnRequestBody**](ColumnRequestBody.md) | A navigable field value. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

