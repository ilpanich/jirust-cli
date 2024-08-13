# \IssueLinkTypesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_issue_link_type**](IssueLinkTypesApi.md#create_issue_link_type) | **POST** /rest/api/3/issueLinkType | Create issue link type
[**delete_issue_link_type**](IssueLinkTypesApi.md#delete_issue_link_type) | **DELETE** /rest/api/3/issueLinkType/{issueLinkTypeId} | Delete issue link type
[**get_issue_link_type**](IssueLinkTypesApi.md#get_issue_link_type) | **GET** /rest/api/3/issueLinkType/{issueLinkTypeId} | Get issue link type
[**get_issue_link_types**](IssueLinkTypesApi.md#get_issue_link_types) | **GET** /rest/api/3/issueLinkType | Get issue link types
[**update_issue_link_type**](IssueLinkTypesApi.md#update_issue_link_type) | **PUT** /rest/api/3/issueLinkType/{issueLinkTypeId} | Update issue link type



## create_issue_link_type

> models::IssueLinkType create_issue_link_type(issue_link_type)
Create issue link type

Creates an issue link type. Use this operation to create descriptions of the reasons why issues are linked. The issue link type consists of a name and descriptions for a link's inward and outward relationships.  To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_link_type** | [**IssueLinkType**](IssueLinkType.md) |  | [required] |

### Return type

[**models::IssueLinkType**](IssueLinkType.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_issue_link_type

> delete_issue_link_type(issue_link_type_id)
Delete issue link type

Deletes an issue link type.  To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_link_type_id** | **String** | The ID of the issue link type. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_link_type

> models::IssueLinkType get_issue_link_type(issue_link_type_id)
Get issue link type

Returns an issue link type.  To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project in the site.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_link_type_id** | **String** | The ID of the issue link type. | [required] |

### Return type

[**models::IssueLinkType**](IssueLinkType.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_link_types

> models::IssueLinkTypes get_issue_link_types()
Get issue link types

Returns a list of all issue link types.  To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project in the site.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IssueLinkTypes**](IssueLinkTypes.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_issue_link_type

> models::IssueLinkType update_issue_link_type(issue_link_type_id, issue_link_type)
Update issue link type

Updates an issue link type.  To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_link_type_id** | **String** | The ID of the issue link type. | [required] |
**issue_link_type** | [**IssueLinkType**](IssueLinkType.md) |  | [required] |

### Return type

[**models::IssueLinkType**](IssueLinkType.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

