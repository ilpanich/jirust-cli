# \IssueCustomFieldValuesAppsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_custom_field_value**](IssueCustomFieldValuesAppsApi.md#update_custom_field_value) | **PUT** /rest/api/3/app/field/{fieldIdOrKey}/value | Update custom field value
[**update_multiple_custom_field_values**](IssueCustomFieldValuesAppsApi.md#update_multiple_custom_field_values) | **POST** /rest/api/3/app/field/value | Update custom fields



## update_custom_field_value

> serde_json::Value update_custom_field_value(field_id_or_key, custom_field_value_update_details, generate_changelog)
Update custom field value

Updates the value of a custom field on one or more issues.  Apps can only perform this operation on [custom fields](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/) and [custom field types](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field-type/) declared in their own manifests.  **[Permissions](#permissions) required:** Only the app that owns the custom field or custom field type can update its values with this operation.  The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id_or_key** | **String** | The ID or key of the custom field. For example, `customfield_10010`. | [required] |
**custom_field_value_update_details** | [**CustomFieldValueUpdateDetails**](CustomFieldValueUpdateDetails.md) |  | [required] |
**generate_changelog** | Option<**bool**> | Whether to generate a changelog for this update. |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_multiple_custom_field_values

> serde_json::Value update_multiple_custom_field_values(multiple_custom_field_values_update_details, generate_changelog)
Update custom fields

Updates the value of one or more custom fields on one or more issues. Combinations of custom field and issue should be unique within the request.  Apps can only perform this operation on [custom fields](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/) and [custom field types](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field-type/) declared in their own manifests.  **[Permissions](#permissions) required:** Only the app that owns the custom field or custom field type can update its values with this operation.  The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**multiple_custom_field_values_update_details** | [**MultipleCustomFieldValuesUpdateDetails**](MultipleCustomFieldValuesUpdateDetails.md) |  | [required] |
**generate_changelog** | Option<**bool**> | Whether to generate a changelog for this update. |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

