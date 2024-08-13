# \IssueCustomFieldOptionsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_field_option**](IssueCustomFieldOptionsApi.md#create_custom_field_option) | **POST** /rest/api/3/field/{fieldId}/context/{contextId}/option | Create custom field options (context)
[**delete_custom_field_option**](IssueCustomFieldOptionsApi.md#delete_custom_field_option) | **DELETE** /rest/api/3/field/{fieldId}/context/{contextId}/option/{optionId} | Delete custom field options (context)
[**get_custom_field_option**](IssueCustomFieldOptionsApi.md#get_custom_field_option) | **GET** /rest/api/3/customFieldOption/{id} | Get custom field option
[**get_options_for_context**](IssueCustomFieldOptionsApi.md#get_options_for_context) | **GET** /rest/api/3/field/{fieldId}/context/{contextId}/option | Get custom field options (context)
[**reorder_custom_field_options**](IssueCustomFieldOptionsApi.md#reorder_custom_field_options) | **PUT** /rest/api/3/field/{fieldId}/context/{contextId}/option/move | Reorder custom field options (context)
[**replace_custom_field_option**](IssueCustomFieldOptionsApi.md#replace_custom_field_option) | **DELETE** /rest/api/3/field/{fieldId}/context/{contextId}/option/{optionId}/issue | Replace custom field options
[**update_custom_field_option**](IssueCustomFieldOptionsApi.md#update_custom_field_option) | **PUT** /rest/api/3/field/{fieldId}/context/{contextId}/option | Update custom field options (context)



## create_custom_field_option

> models::CustomFieldCreatedContextOptionsList create_custom_field_option(field_id, context_id, bulk_custom_field_option_create_request)
Create custom field options (context)

Creates options and, where the custom select field is of the type Select List (cascading), cascading options for a custom select field. The options are added to a context of the field.  The maximum number of options that can be created per request is 1000 and each field can have a maximum of 10000 options.  This operation works for custom field options created in Jira or the operations from this resource. **To work with issue field select list options created for Connect apps use the [Issue custom field options (apps)](#api-group-issue-custom-field-options--apps-) operations.**  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**bulk_custom_field_option_create_request** | [**BulkCustomFieldOptionCreateRequest**](BulkCustomFieldOptionCreateRequest.md) |  | [required] |

### Return type

[**models::CustomFieldCreatedContextOptionsList**](CustomFieldCreatedContextOptionsList.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_field_option

> delete_custom_field_option(field_id, context_id, option_id)
Delete custom field options (context)

Deletes a custom field option.  Options with cascading options cannot be deleted without deleting the cascading options first.  This operation works for custom field options created in Jira or the operations from this resource. **To work with issue field select list options created for Connect apps use the [Issue custom field options (apps)](#api-group-issue-custom-field-options--apps-) operations.**  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context from which an option should be deleted. | [required] |
**option_id** | **i64** | The ID of the option to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_field_option

> models::CustomFieldOption get_custom_field_option(id)
Get custom field option

Returns a custom field option. For example, an option in a select list.  Note that this operation **only works for issue field select list options created in Jira or using operations from the [Issue custom field options](#api-group-Issue-custom-field-options) resource**, it cannot be used with issue field select list options created by Connect apps.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** The custom field option is returned as follows:   *  if the user has the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  if the user has the *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for at least one project the custom field is used in, and the field is visible in at least one layout the user has permission to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the custom field option. | [required] |

### Return type

[**models::CustomFieldOption**](CustomFieldOption.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_options_for_context

> models::PageBeanCustomFieldContextOption get_options_for_context(field_id, context_id, option_id, only_options, start_at, max_results)
Get custom field options (context)

Returns a [paginated](#pagination) list of all custom field option for a context. Options are returned first then cascading options, in the order they display in Jira.  This operation works for custom field options created in Jira or the operations from this resource. **To work with issue field select list options created for Connect apps use the [Issue custom field options (apps)](#api-group-issue-custom-field-options--apps-) operations.**  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**option_id** | Option<**i64**> | The ID of the option. |  |
**only_options** | Option<**bool**> | Whether only options are returned. |  |[default to false]
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]

### Return type

[**models::PageBeanCustomFieldContextOption**](PageBeanCustomFieldContextOption.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reorder_custom_field_options

> serde_json::Value reorder_custom_field_options(field_id, context_id, order_of_custom_field_options)
Reorder custom field options (context)

Changes the order of custom field options or cascading options in a context.  This operation works for custom field options created in Jira or the operations from this resource. **To work with issue field select list options created for Connect apps use the [Issue custom field options (apps)](#api-group-issue-custom-field-options--apps-) operations.**  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**order_of_custom_field_options** | [**OrderOfCustomFieldOptions**](OrderOfCustomFieldOptions.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_custom_field_option

> replace_custom_field_option(field_id, option_id, context_id, replace_with, jql)
Replace custom field options

Replaces the options of a custom field.  Note that this operation **only works for issue field select list options created in Jira or using operations from the [Issue custom field options](#api-group-Issue-custom-field-options) resource**, it cannot be used with issue field select list options created by Connect or Forge apps.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**option_id** | **i64** | The ID of the option to be deselected. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**replace_with** | Option<**i64**> | The ID of the option that will replace the currently selected option. |  |
**jql** | Option<**String**> | A JQL query that specifies the issues to be updated. For example, *project=10000*. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_field_option

> models::CustomFieldUpdatedContextOptionsList update_custom_field_option(field_id, context_id, bulk_custom_field_option_update_request)
Update custom field options (context)

Updates the options of a custom field.  If any of the options are not found, no options are updated. Options where the values in the request match the current values aren't updated and aren't reported in the response.  Note that this operation **only works for issue field select list options created in Jira or using operations from the [Issue custom field options](#api-group-Issue-custom-field-options) resource**, it cannot be used with issue field select list options created by Connect apps.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**bulk_custom_field_option_update_request** | [**BulkCustomFieldOptionUpdateRequest**](BulkCustomFieldOptionUpdateRequest.md) |  | [required] |

### Return type

[**models::CustomFieldUpdatedContextOptionsList**](CustomFieldUpdatedContextOptionsList.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

