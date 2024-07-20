# \IssueFieldConfigurationsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_field_configuration_scheme_to_project**](IssueFieldConfigurationsApi.md#assign_field_configuration_scheme_to_project) | **PUT** /rest/api/3/fieldconfigurationscheme/project | Assign field configuration scheme to project
[**create_field_configuration**](IssueFieldConfigurationsApi.md#create_field_configuration) | **POST** /rest/api/3/fieldconfiguration | Create field configuration
[**create_field_configuration_scheme**](IssueFieldConfigurationsApi.md#create_field_configuration_scheme) | **POST** /rest/api/3/fieldconfigurationscheme | Create field configuration scheme
[**delete_field_configuration**](IssueFieldConfigurationsApi.md#delete_field_configuration) | **DELETE** /rest/api/3/fieldconfiguration/{id} | Delete field configuration
[**delete_field_configuration_scheme**](IssueFieldConfigurationsApi.md#delete_field_configuration_scheme) | **DELETE** /rest/api/3/fieldconfigurationscheme/{id} | Delete field configuration scheme
[**get_all_field_configuration_schemes**](IssueFieldConfigurationsApi.md#get_all_field_configuration_schemes) | **GET** /rest/api/3/fieldconfigurationscheme | Get all field configuration schemes
[**get_all_field_configurations**](IssueFieldConfigurationsApi.md#get_all_field_configurations) | **GET** /rest/api/3/fieldconfiguration | Get all field configurations
[**get_field_configuration_items**](IssueFieldConfigurationsApi.md#get_field_configuration_items) | **GET** /rest/api/3/fieldconfiguration/{id}/fields | Get field configuration items
[**get_field_configuration_scheme_mappings**](IssueFieldConfigurationsApi.md#get_field_configuration_scheme_mappings) | **GET** /rest/api/3/fieldconfigurationscheme/mapping | Get field configuration issue type items
[**get_field_configuration_scheme_project_mapping**](IssueFieldConfigurationsApi.md#get_field_configuration_scheme_project_mapping) | **GET** /rest/api/3/fieldconfigurationscheme/project | Get field configuration schemes for projects
[**remove_issue_types_from_global_field_configuration_scheme**](IssueFieldConfigurationsApi.md#remove_issue_types_from_global_field_configuration_scheme) | **POST** /rest/api/3/fieldconfigurationscheme/{id}/mapping/delete | Remove issue types from field configuration scheme
[**set_field_configuration_scheme_mapping**](IssueFieldConfigurationsApi.md#set_field_configuration_scheme_mapping) | **PUT** /rest/api/3/fieldconfigurationscheme/{id}/mapping | Assign issue types to field configurations
[**update_field_configuration**](IssueFieldConfigurationsApi.md#update_field_configuration) | **PUT** /rest/api/3/fieldconfiguration/{id} | Update field configuration
[**update_field_configuration_items**](IssueFieldConfigurationsApi.md#update_field_configuration_items) | **PUT** /rest/api/3/fieldconfiguration/{id}/fields | Update field configuration items
[**update_field_configuration_scheme**](IssueFieldConfigurationsApi.md#update_field_configuration_scheme) | **PUT** /rest/api/3/fieldconfigurationscheme/{id} | Update field configuration scheme



## assign_field_configuration_scheme_to_project

> serde_json::Value assign_field_configuration_scheme_to_project(field_configuration_scheme_project_association)
Assign field configuration scheme to project

Assigns a field configuration scheme to a project. If the field configuration scheme ID is `null`, the operation assigns the default field configuration scheme.  Field configuration schemes can only be assigned to classic projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_configuration_scheme_project_association** | [**FieldConfigurationSchemeProjectAssociation**](FieldConfigurationSchemeProjectAssociation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_field_configuration

> models::FieldConfiguration create_field_configuration(field_configuration_details)
Create field configuration

Creates a field configuration. The field configuration is created with the same field properties as the default configuration, with all the fields being optional.  This operation can only create configurations for use in company-managed (classic) projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_configuration_details** | [**FieldConfigurationDetails**](FieldConfigurationDetails.md) |  | [required] |

### Return type

[**models::FieldConfiguration**](FieldConfiguration.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_field_configuration_scheme

> models::FieldConfigurationScheme create_field_configuration_scheme(update_field_configuration_scheme_details)
Create field configuration scheme

Creates a field configuration scheme.  This operation can only create field configuration schemes used in company-managed (classic) projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_field_configuration_scheme_details** | [**UpdateFieldConfigurationSchemeDetails**](UpdateFieldConfigurationSchemeDetails.md) | The details of the field configuration scheme. | [required] |

### Return type

[**models::FieldConfigurationScheme**](FieldConfigurationScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_field_configuration

> serde_json::Value delete_field_configuration(id)
Delete field configuration

Deletes a field configuration.  This operation can only delete configurations used in company-managed (classic) projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field configuration. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_field_configuration_scheme

> serde_json::Value delete_field_configuration_scheme(id)
Delete field configuration scheme

Deletes a field configuration scheme.  This operation can only delete field configuration schemes used in company-managed (classic) projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field configuration scheme. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_field_configuration_schemes

> models::PageBeanFieldConfigurationScheme get_all_field_configuration_schemes(start_at, max_results, id)
Get all field configuration schemes

Returns a [paginated](#pagination) list of field configuration schemes.  Only field configuration schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<i64>**](i64.md)> | The list of field configuration scheme IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=10000&id=10001`. |  |

### Return type

[**models::PageBeanFieldConfigurationScheme**](PageBeanFieldConfigurationScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_field_configurations

> models::PageBeanFieldConfigurationDetails get_all_field_configurations(start_at, max_results, id, is_default, query)
Get all field configurations

Returns a [paginated](#pagination) list of field configurations. The list can be for all field configurations or a subset determined by any combination of these criteria:   *  a list of field configuration item IDs.  *  whether the field configuration is a default.  *  whether the field configuration name or description contains a query string.  Only field configurations used in company-managed (classic) projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<i64>**](i64.md)> | The list of field configuration IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=10000&id=10001`. |  |
**is_default** | Option<**bool**> | If *true* returns default field configurations only. |  |[default to false]
**query** | Option<**String**> | The query string used to match against field configuration names and descriptions. |  |[default to ]

### Return type

[**models::PageBeanFieldConfigurationDetails**](PageBeanFieldConfigurationDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_field_configuration_items

> models::PageBeanFieldConfigurationItem get_field_configuration_items(id, start_at, max_results)
Get field configuration items

Returns a [paginated](#pagination) list of all fields for a configuration.  Only the fields from configurations used in company-managed (classic) projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field configuration. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanFieldConfigurationItem**](PageBeanFieldConfigurationItem.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_field_configuration_scheme_mappings

> models::PageBeanFieldConfigurationIssueTypeItem get_field_configuration_scheme_mappings(start_at, max_results, field_configuration_scheme_id)
Get field configuration issue type items

Returns a [paginated](#pagination) list of field configuration issue type items.  Only items used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**field_configuration_scheme_id** | Option<[**Vec<i64>**](i64.md)> | The list of field configuration scheme IDs. To include multiple field configuration schemes separate IDs with ampersand: `fieldConfigurationSchemeId=10000&fieldConfigurationSchemeId=10001`. |  |

### Return type

[**models::PageBeanFieldConfigurationIssueTypeItem**](PageBeanFieldConfigurationIssueTypeItem.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_field_configuration_scheme_project_mapping

> models::PageBeanFieldConfigurationSchemeProjects get_field_configuration_scheme_project_mapping(project_id, start_at, max_results)
Get field configuration schemes for projects

Returns a [paginated](#pagination) list of field configuration schemes and, for each scheme, a list of the projects that use it.  The list is sorted by field configuration scheme ID. The first item contains the list of project IDs assigned to the default field configuration scheme.  Only field configuration schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**Vec<i64>**](i64.md) | The list of project IDs. To include multiple projects, separate IDs with ampersand: `projectId=10000&projectId=10001`. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanFieldConfigurationSchemeProjects**](PageBeanFieldConfigurationSchemeProjects.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_issue_types_from_global_field_configuration_scheme

> serde_json::Value remove_issue_types_from_global_field_configuration_scheme(id, issue_type_ids_to_remove)
Remove issue types from field configuration scheme

Removes issue types from the field configuration scheme.  This operation can only modify field configuration schemes used in company-managed (classic) projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field configuration scheme. | [required] |
**issue_type_ids_to_remove** | [**IssueTypeIdsToRemove**](IssueTypeIdsToRemove.md) | The issue type IDs to remove. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_field_configuration_scheme_mapping

> serde_json::Value set_field_configuration_scheme_mapping(id, associate_field_configurations_with_issue_types_request)
Assign issue types to field configurations

Assigns issue types to field configurations on field configuration scheme.  This operation can only modify field configuration schemes used in company-managed (classic) projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field configuration scheme. | [required] |
**associate_field_configurations_with_issue_types_request** | [**AssociateFieldConfigurationsWithIssueTypesRequest**](AssociateFieldConfigurationsWithIssueTypesRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_field_configuration

> serde_json::Value update_field_configuration(id, field_configuration_details)
Update field configuration

Updates a field configuration. The name and the description provided in the request override the existing values.  This operation can only update configurations used in company-managed (classic) projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field configuration. | [required] |
**field_configuration_details** | [**FieldConfigurationDetails**](FieldConfigurationDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_field_configuration_items

> serde_json::Value update_field_configuration_items(id, field_configuration_items_details)
Update field configuration items

Updates fields in a field configuration. The properties of the field configuration fields provided override the existing values.  This operation can only update field configurations used in company-managed (classic) projects.  The operation can set the renderer for text fields to the default text renderer (`text-renderer`) or wiki style renderer (`wiki-renderer`). However, the renderer cannot be updated for fields using the autocomplete renderer (`autocomplete-renderer`).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field configuration. | [required] |
**field_configuration_items_details** | [**FieldConfigurationItemsDetails**](FieldConfigurationItemsDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_field_configuration_scheme

> serde_json::Value update_field_configuration_scheme(id, update_field_configuration_scheme_details)
Update field configuration scheme

Updates a field configuration scheme.  This operation can only update field configuration schemes used in company-managed (classic) projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field configuration scheme. | [required] |
**update_field_configuration_scheme_details** | [**UpdateFieldConfigurationSchemeDetails**](UpdateFieldConfigurationSchemeDetails.md) | The details of the field configuration scheme. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

