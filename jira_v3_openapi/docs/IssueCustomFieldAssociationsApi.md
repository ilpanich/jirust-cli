# \IssueCustomFieldAssociationsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_associations**](IssueCustomFieldAssociationsApi.md#create_associations) | **PUT** /rest/api/3/field/association | Create associations
[**remove_associations**](IssueCustomFieldAssociationsApi.md#remove_associations) | **DELETE** /rest/api/3/field/association | Remove associations



## create_associations

> serde_json::Value create_associations(field_associations_request)
Create associations

Associates fields with projects.  Fields will be associated with each issue type on the requested projects.  Fields will be associated with all projects that share the same field configuration which the provided projects are using. This means that while the field will be associated with the requested projects, it will also be associated with any other projects that share the same field configuration.  If a success response is returned it means that the field association has been created in any applicable contexts where it wasn't already present.  Up to 50 fields and up to 100 projects can be associated in a single request. If more fields or projects are provided a 400 response will be returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_associations_request** | [**FieldAssociationsRequest**](FieldAssociationsRequest.md) | Payload containing the fields to associate and the projects to associate them to. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_associations

> serde_json::Value remove_associations(field_associations_request)
Remove associations

Unassociates a set of fields with a project and issue type context.  Fields will be unassociated with all projects/issue types that share the same field configuration which the provided project and issue types are using. This means that while the field will be unassociated with the provided project and issue types, it will also be unassociated with any other projects and issue types that share the same field configuration.  If a success response is returned it means that the field association has been removed in any applicable contexts where it was present.  Up to 50 fields and up to 100 projects and issue types can be unassociated in a single request. If more fields or projects are provided a 400 response will be returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_associations_request** | [**FieldAssociationsRequest**](FieldAssociationsRequest.md) | Payload containing the fields to uassociate and the projects and issue types to unassociate them to. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

