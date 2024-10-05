# \IssueCustomFieldConfigurationAppsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_custom_field_configuration**](IssueCustomFieldConfigurationAppsApi.md#get_custom_field_configuration) | **GET** /rest/api/3/app/field/{fieldIdOrKey}/context/configuration | Get custom field configurations
[**get_custom_fields_configurations**](IssueCustomFieldConfigurationAppsApi.md#get_custom_fields_configurations) | **POST** /rest/api/3/app/field/context/configuration/list | Bulk get custom field configurations
[**update_custom_field_configuration**](IssueCustomFieldConfigurationAppsApi.md#update_custom_field_configuration) | **PUT** /rest/api/3/app/field/{fieldIdOrKey}/context/configuration | Update custom field configurations



## get_custom_field_configuration

> models::PageBeanContextualConfiguration get_custom_field_configuration(field_id_or_key, id, field_context_id, issue_id, project_key_or_id, issue_type_id, start_at, max_results)
Get custom field configurations

Returns a [paginated](#pagination) list of configurations for a custom field of a [type](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field-type/) created by a [Forge app](https://developer.atlassian.com/platform/forge/).  The result can be filtered by one of these criteria:   *  `id`.  *  `fieldContextId`.  *  `issueId`.  *  `projectKeyOrId` and `issueTypeId`.  Otherwise, all configurations are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the Forge app that provided the custom field type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id_or_key** | **String** | The ID or key of the custom field, for example `customfield_10000`. | [required] |
**id** | Option<[**Vec<i64>**](i64.md)> | The list of configuration IDs. To include multiple configurations, separate IDs with an ampersand: `id=10000&id=10001`. Can't be provided with `fieldContextId`, `issueId`, `projectKeyOrId`, or `issueTypeId`. |  |
**field_context_id** | Option<[**Vec<i64>**](i64.md)> | The list of field context IDs. To include multiple field contexts, separate IDs with an ampersand: `fieldContextId=10000&fieldContextId=10001`. Can't be provided with `id`, `issueId`, `projectKeyOrId`, or `issueTypeId`. |  |
**issue_id** | Option<**i64**> | The ID of the issue to filter results by. If the issue doesn't exist, an empty list is returned. Can't be provided with `projectKeyOrId`, or `issueTypeId`. |  |
**project_key_or_id** | Option<**String**> | The ID or key of the project to filter results by. Must be provided with `issueTypeId`. Can't be provided with `issueId`. |  |
**issue_type_id** | Option<**String**> | The ID of the issue type to filter results by. Must be provided with `projectKeyOrId`. Can't be provided with `issueId`. |  |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]

### Return type

[**models::PageBeanContextualConfiguration**](PageBeanContextualConfiguration.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_fields_configurations

> models::PageBeanBulkContextualConfiguration get_custom_fields_configurations(configurations_list_parameters, id, field_context_id, issue_id, project_key_or_id, issue_type_id, start_at, max_results)
Bulk get custom field configurations

Returns a [paginated](#pagination) list of configurations for list of custom fields of a [type](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field-type/) created by a [Forge app](https://developer.atlassian.com/platform/forge/).  The result can be filtered by one of these criteria:   *  `id`.  *  `fieldContextId`.  *  `issueId`.  *  `projectKeyOrId` and `issueTypeId`.  Otherwise, all configurations for the provided list of custom fields are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the Forge app that provided the custom field type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configurations_list_parameters** | [**ConfigurationsListParameters**](ConfigurationsListParameters.md) |  | [required] |
**id** | Option<[**Vec<i64>**](i64.md)> | The list of configuration IDs. To include multiple configurations, separate IDs with an ampersand: `id=10000&id=10001`. Can't be provided with `fieldContextId`, `issueId`, `projectKeyOrId`, or `issueTypeId`. |  |
**field_context_id** | Option<[**Vec<i64>**](i64.md)> | The list of field context IDs. To include multiple field contexts, separate IDs with an ampersand: `fieldContextId=10000&fieldContextId=10001`. Can't be provided with `id`, `issueId`, `projectKeyOrId`, or `issueTypeId`. |  |
**issue_id** | Option<**i64**> | The ID of the issue to filter results by. If the issue doesn't exist, an empty list is returned. Can't be provided with `projectKeyOrId`, or `issueTypeId`. |  |
**project_key_or_id** | Option<**String**> | The ID or key of the project to filter results by. Must be provided with `issueTypeId`. Can't be provided with `issueId`. |  |
**issue_type_id** | Option<**String**> | The ID of the issue type to filter results by. Must be provided with `projectKeyOrId`. Can't be provided with `issueId`. |  |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]

### Return type

[**models::PageBeanBulkContextualConfiguration**](PageBeanBulkContextualConfiguration.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_field_configuration

> serde_json::Value update_custom_field_configuration(field_id_or_key, custom_field_configurations)
Update custom field configurations

Update the configuration for contexts of a custom field of a [type](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field-type/) created by a [Forge app](https://developer.atlassian.com/platform/forge/).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the Forge app that created the custom field type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id_or_key** | **String** | The ID or key of the custom field, for example `customfield_10000`. | [required] |
**custom_field_configurations** | [**CustomFieldConfigurations**](CustomFieldConfigurations.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

