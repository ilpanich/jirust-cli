# \IssueCustomFieldContextsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_issue_types_to_context**](IssueCustomFieldContextsApi.md#add_issue_types_to_context) | **PUT** /rest/api/3/field/{fieldId}/context/{contextId}/issuetype | Add issue types to context
[**assign_projects_to_custom_field_context**](IssueCustomFieldContextsApi.md#assign_projects_to_custom_field_context) | **PUT** /rest/api/3/field/{fieldId}/context/{contextId}/project | Assign custom field context to projects
[**create_custom_field_context**](IssueCustomFieldContextsApi.md#create_custom_field_context) | **POST** /rest/api/3/field/{fieldId}/context | Create custom field context
[**delete_custom_field_context**](IssueCustomFieldContextsApi.md#delete_custom_field_context) | **DELETE** /rest/api/3/field/{fieldId}/context/{contextId} | Delete custom field context
[**get_contexts_for_field**](IssueCustomFieldContextsApi.md#get_contexts_for_field) | **GET** /rest/api/3/field/{fieldId}/context | Get custom field contexts
[**get_custom_field_contexts_for_projects_and_issue_types**](IssueCustomFieldContextsApi.md#get_custom_field_contexts_for_projects_and_issue_types) | **POST** /rest/api/3/field/{fieldId}/context/mapping | Get custom field contexts for projects and issue types
[**get_default_values**](IssueCustomFieldContextsApi.md#get_default_values) | **GET** /rest/api/3/field/{fieldId}/context/defaultValue | Get custom field contexts default values
[**get_issue_type_mappings_for_contexts**](IssueCustomFieldContextsApi.md#get_issue_type_mappings_for_contexts) | **GET** /rest/api/3/field/{fieldId}/context/issuetypemapping | Get issue types for custom field context
[**get_project_context_mapping**](IssueCustomFieldContextsApi.md#get_project_context_mapping) | **GET** /rest/api/3/field/{fieldId}/context/projectmapping | Get project mappings for custom field context
[**remove_custom_field_context_from_projects**](IssueCustomFieldContextsApi.md#remove_custom_field_context_from_projects) | **POST** /rest/api/3/field/{fieldId}/context/{contextId}/project/remove | Remove custom field context from projects
[**remove_issue_types_from_context**](IssueCustomFieldContextsApi.md#remove_issue_types_from_context) | **POST** /rest/api/3/field/{fieldId}/context/{contextId}/issuetype/remove | Remove issue types from context
[**set_default_values**](IssueCustomFieldContextsApi.md#set_default_values) | **PUT** /rest/api/3/field/{fieldId}/context/defaultValue | Set custom field contexts default values
[**update_custom_field_context**](IssueCustomFieldContextsApi.md#update_custom_field_context) | **PUT** /rest/api/3/field/{fieldId}/context/{contextId} | Update custom field context



## add_issue_types_to_context

> serde_json::Value add_issue_types_to_context(field_id, context_id, issue_type_ids)
Add issue types to context

Adds issue types to a custom field context, appending the issue types to the issue types list.  A custom field context without any issue types applies to all issue types. Adding issue types to such a custom field context would result in it applying to only the listed issue types.  If any of the issue types exists in the custom field context, the operation fails and no issue types are added.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**issue_type_ids** | [**IssueTypeIds**](IssueTypeIds.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_projects_to_custom_field_context

> serde_json::Value assign_projects_to_custom_field_context(field_id, context_id, project_ids)
Assign custom field context to projects

Assigns a custom field context to projects.  If any project in the request is assigned to any context of the custom field, the operation fails.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**project_ids** | [**ProjectIds**](ProjectIds.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_custom_field_context

> models::CreateCustomFieldContext create_custom_field_context(field_id, create_custom_field_context)
Create custom field context

Creates a custom field context.  If `projectIds` is empty, a global context is created. A global context is one that applies to all project. If `issueTypeIds` is empty, the context applies to all issue types.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**create_custom_field_context** | [**CreateCustomFieldContext**](CreateCustomFieldContext.md) |  | [required] |

### Return type

[**models::CreateCustomFieldContext**](CreateCustomFieldContext.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_field_context

> serde_json::Value delete_custom_field_context(field_id, context_id)
Delete custom field context

Deletes a [ custom field context](https://confluence.atlassian.com/adminjiracloud/what-are-custom-field-contexts-991923859.html).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contexts_for_field

> models::PageBeanCustomFieldContext get_contexts_for_field(field_id, is_any_issue_type, is_global_context, context_id, start_at, max_results)
Get custom field contexts

Returns a [paginated](#pagination) list of [ contexts](https://confluence.atlassian.com/adminjiracloud/what-are-custom-field-contexts-991923859.html) for a custom field. Contexts can be returned as follows:   *  With no other parameters set, all contexts.  *  By defining `id` only, all contexts from the list of IDs.  *  By defining `isAnyIssueType`, limit the list of contexts returned to either those that apply to all issue types (true) or those that apply to only a subset of issue types (false)  *  By defining `isGlobalContext`, limit the list of contexts return to either those that apply to all projects (global contexts) (true) or those that apply to only a subset of projects (false).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**is_any_issue_type** | Option<**bool**> | Whether to return contexts that apply to all issue types. |  |
**is_global_context** | Option<**bool**> | Whether to return contexts that apply to all projects. |  |
**context_id** | Option<[**Vec<i64>**](i64.md)> | The list of context IDs. To include multiple contexts, separate IDs with ampersand: `contextId=10000&contextId=10001`. |  |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanCustomFieldContext**](PageBeanCustomFieldContext.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_field_contexts_for_projects_and_issue_types

> models::PageBeanContextForProjectAndIssueType get_custom_field_contexts_for_projects_and_issue_types(field_id, project_issue_type_mappings, start_at, max_results)
Get custom field contexts for projects and issue types

Returns a [paginated](#pagination) list of project and issue type mappings and, for each mapping, the ID of a [custom field context](https://confluence.atlassian.com/x/k44fOw) that applies to the project and issue type.  If there is no custom field context assigned to the project then, if present, the custom field context that applies to all projects is returned if it also applies to the issue type or all issue types. If a custom field context is not found, the returned custom field context ID is `null`.  Duplicate project and issue type mappings cannot be provided in the request.  The order of the returned values is the same as provided in the request.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**project_issue_type_mappings** | [**ProjectIssueTypeMappings**](ProjectIssueTypeMappings.md) | The list of project and issue type mappings. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanContextForProjectAndIssueType**](PageBeanContextForProjectAndIssueType.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_values

> models::PageBeanCustomFieldContextDefaultValue get_default_values(field_id, context_id, start_at, max_results)
Get custom field contexts default values

Returns a [paginated](#pagination) list of defaults for a custom field. The results can be filtered by `contextId`, otherwise all values are returned. If no defaults are set for a context, nothing is returned.   The returned object depends on type of the custom field:   *  `CustomFieldContextDefaultValueDate` (type `datepicker`) for date fields.  *  `CustomFieldContextDefaultValueDateTime` (type `datetimepicker`) for date-time fields.  *  `CustomFieldContextDefaultValueSingleOption` (type `option.single`) for single choice select lists and radio buttons.  *  `CustomFieldContextDefaultValueMultipleOption` (type `option.multiple`) for multiple choice select lists and checkboxes.  *  `CustomFieldContextDefaultValueCascadingOption` (type `option.cascading`) for cascading select lists.  *  `CustomFieldContextSingleUserPickerDefaults` (type `single.user.select`) for single users.  *  `CustomFieldContextDefaultValueMultiUserPicker` (type `multi.user.select`) for user lists.  *  `CustomFieldContextDefaultValueSingleGroupPicker` (type `grouppicker.single`) for single choice group pickers.  *  `CustomFieldContextDefaultValueMultipleGroupPicker` (type `grouppicker.multiple`) for multiple choice group pickers.  *  `CustomFieldContextDefaultValueURL` (type `url`) for URLs.  *  `CustomFieldContextDefaultValueProject` (type `project`) for project pickers.  *  `CustomFieldContextDefaultValueFloat` (type `float`) for floats (floating-point numbers).  *  `CustomFieldContextDefaultValueLabels` (type `labels`) for labels.  *  `CustomFieldContextDefaultValueTextField` (type `textfield`) for text fields.  *  `CustomFieldContextDefaultValueTextArea` (type `textarea`) for text area fields.  *  `CustomFieldContextDefaultValueReadOnly` (type `readonly`) for read only (text) fields.  *  `CustomFieldContextDefaultValueMultipleVersion` (type `version.multiple`) for single choice version pickers.  *  `CustomFieldContextDefaultValueSingleVersion` (type `version.single`) for multiple choice version pickers.  Forge custom fields [types](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field-type/#data-types) are also supported, returning:   *  `CustomFieldContextDefaultValueForgeStringFieldBean` (type `forge.string`) for Forge string fields.  *  `CustomFieldContextDefaultValueForgeMultiStringFieldBean` (type `forge.string.list`) for Forge string collection fields.  *  `CustomFieldContextDefaultValueForgeObjectFieldBean` (type `forge.object`) for Forge object fields.  *  `CustomFieldContextDefaultValueForgeDateTimeFieldBean` (type `forge.datetime`) for Forge date-time fields.  *  `CustomFieldContextDefaultValueForgeGroupFieldBean` (type `forge.group`) for Forge group fields.  *  `CustomFieldContextDefaultValueForgeMultiGroupFieldBean` (type `forge.group.list`) for Forge group collection fields.  *  `CustomFieldContextDefaultValueForgeNumberFieldBean` (type `forge.number`) for Forge number fields.  *  `CustomFieldContextDefaultValueForgeUserFieldBean` (type `forge.user`) for Forge user fields.  *  `CustomFieldContextDefaultValueForgeMultiUserFieldBean` (type `forge.user.list`) for Forge user collection fields.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field, for example `customfield\\_10000`. | [required] |
**context_id** | Option<[**Vec<i64>**](i64.md)> | The IDs of the contexts. |  |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanCustomFieldContextDefaultValue**](PageBeanCustomFieldContextDefaultValue.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_type_mappings_for_contexts

> models::PageBeanIssueTypeToContextMapping get_issue_type_mappings_for_contexts(field_id, context_id, start_at, max_results)
Get issue types for custom field context

Returns a [paginated](#pagination) list of context to issue type mappings for a custom field. Mappings are returned for all contexts or a list of contexts. Mappings are ordered first by context ID and then by issue type ID.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | Option<[**Vec<i64>**](i64.md)> | The ID of the context. To include multiple contexts, provide an ampersand-separated list. For example, `contextId=10001&contextId=10002`. |  |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanIssueTypeToContextMapping**](PageBeanIssueTypeToContextMapping.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_context_mapping

> models::PageBeanCustomFieldContextProjectMapping get_project_context_mapping(field_id, context_id, start_at, max_results)
Get project mappings for custom field context

Returns a [paginated](#pagination) list of context to project mappings for a custom field. The result can be filtered by `contextId`. Otherwise, all mappings are returned. Invalid IDs are ignored.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field, for example `customfield\\_10000`. | [required] |
**context_id** | Option<[**Vec<i64>**](i64.md)> | The list of context IDs. To include multiple context, separate IDs with ampersand: `contextId=10000&contextId=10001`. |  |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanCustomFieldContextProjectMapping**](PageBeanCustomFieldContextProjectMapping.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_custom_field_context_from_projects

> serde_json::Value remove_custom_field_context_from_projects(field_id, context_id, project_ids)
Remove custom field context from projects

Removes a custom field context from projects.  A custom field context without any projects applies to all projects. Removing all projects from a custom field context would result in it applying to all projects.  If any project in the request is not assigned to the context, or the operation would result in two global contexts for the field, the operation fails.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**project_ids** | [**ProjectIds**](ProjectIds.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_issue_types_from_context

> serde_json::Value remove_issue_types_from_context(field_id, context_id, issue_type_ids)
Remove issue types from context

Removes issue types from a custom field context.  A custom field context without any issue types applies to all issue types.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**issue_type_ids** | [**IssueTypeIds**](IssueTypeIds.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_values

> serde_json::Value set_default_values(field_id, custom_field_context_default_value_update)
Set custom field contexts default values

Sets default for contexts of a custom field. Default are defined using these objects:   *  `CustomFieldContextDefaultValueDate` (type `datepicker`) for date fields.  *  `CustomFieldContextDefaultValueDateTime` (type `datetimepicker`) for date-time fields.  *  `CustomFieldContextDefaultValueSingleOption` (type `option.single`) for single choice select lists and radio buttons.  *  `CustomFieldContextDefaultValueMultipleOption` (type `option.multiple`) for multiple choice select lists and checkboxes.  *  `CustomFieldContextDefaultValueCascadingOption` (type `option.cascading`) for cascading select lists.  *  `CustomFieldContextSingleUserPickerDefaults` (type `single.user.select`) for single users.  *  `CustomFieldContextDefaultValueMultiUserPicker` (type `multi.user.select`) for user lists.  *  `CustomFieldContextDefaultValueSingleGroupPicker` (type `grouppicker.single`) for single choice group pickers.  *  `CustomFieldContextDefaultValueMultipleGroupPicker` (type `grouppicker.multiple`) for multiple choice group pickers.  *  `CustomFieldContextDefaultValueURL` (type `url`) for URLs.  *  `CustomFieldContextDefaultValueProject` (type `project`) for project pickers.  *  `CustomFieldContextDefaultValueFloat` (type `float`) for floats (floating-point numbers).  *  `CustomFieldContextDefaultValueLabels` (type `labels`) for labels.  *  `CustomFieldContextDefaultValueTextField` (type `textfield`) for text fields.  *  `CustomFieldContextDefaultValueTextArea` (type `textarea`) for text area fields.  *  `CustomFieldContextDefaultValueReadOnly` (type `readonly`) for read only (text) fields.  *  `CustomFieldContextDefaultValueMultipleVersion` (type `version.multiple`) for single choice version pickers.  *  `CustomFieldContextDefaultValueSingleVersion` (type `version.single`) for multiple choice version pickers.  Forge custom fields [types](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field-type/#data-types) are also supported, returning:   *  `CustomFieldContextDefaultValueForgeStringFieldBean` (type `forge.string`) for Forge string fields.  *  `CustomFieldContextDefaultValueForgeMultiStringFieldBean` (type `forge.string.list`) for Forge string collection fields.  *  `CustomFieldContextDefaultValueForgeObjectFieldBean` (type `forge.object`) for Forge object fields.  *  `CustomFieldContextDefaultValueForgeDateTimeFieldBean` (type `forge.datetime`) for Forge date-time fields.  *  `CustomFieldContextDefaultValueForgeGroupFieldBean` (type `forge.group`) for Forge group fields.  *  `CustomFieldContextDefaultValueForgeMultiGroupFieldBean` (type `forge.group.list`) for Forge group collection fields.  *  `CustomFieldContextDefaultValueForgeNumberFieldBean` (type `forge.number`) for Forge number fields.  *  `CustomFieldContextDefaultValueForgeUserFieldBean` (type `forge.user`) for Forge user fields.  *  `CustomFieldContextDefaultValueForgeMultiUserFieldBean` (type `forge.user.list`) for Forge user collection fields.  Only one type of default object can be included in a request. To remove a default for a context, set the default parameter to `null`.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**custom_field_context_default_value_update** | [**CustomFieldContextDefaultValueUpdate**](CustomFieldContextDefaultValueUpdate.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_field_context

> serde_json::Value update_custom_field_context(field_id, context_id, custom_field_context_update_details)
Update custom field context

Updates a [ custom field context](https://confluence.atlassian.com/adminjiracloud/what-are-custom-field-contexts-991923859.html).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the custom field. | [required] |
**context_id** | **i64** | The ID of the context. | [required] |
**custom_field_context_update_details** | [**CustomFieldContextUpdateDetails**](CustomFieldContextUpdateDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

