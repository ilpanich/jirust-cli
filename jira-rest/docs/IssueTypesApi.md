# \IssueTypesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_issue_type**](IssueTypesApi.md#create_issue_type) | **POST** /rest/api/3/issuetype | Create issue type
[**create_issue_type_avatar**](IssueTypesApi.md#create_issue_type_avatar) | **POST** /rest/api/3/issuetype/{id}/avatar2 | Load issue type avatar
[**delete_issue_type**](IssueTypesApi.md#delete_issue_type) | **DELETE** /rest/api/3/issuetype/{id} | Delete issue type
[**get_alternative_issue_types**](IssueTypesApi.md#get_alternative_issue_types) | **GET** /rest/api/3/issuetype/{id}/alternatives | Get alternative issue types
[**get_issue_all_types**](IssueTypesApi.md#get_issue_all_types) | **GET** /rest/api/3/issuetype | Get all issue types for user
[**get_issue_type**](IssueTypesApi.md#get_issue_type) | **GET** /rest/api/3/issuetype/{id} | Get issue type
[**get_issue_types_for_project**](IssueTypesApi.md#get_issue_types_for_project) | **GET** /rest/api/3/issuetype/project | Get issue types for project
[**update_issue_type**](IssueTypesApi.md#update_issue_type) | **PUT** /rest/api/3/issuetype/{id} | Update issue type



## create_issue_type

> models::IssueTypeDetails create_issue_type(issue_type_create_bean)
Create issue type

Creates an issue type and adds it to the default issue type scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_create_bean** | [**IssueTypeCreateBean**](IssueTypeCreateBean.md) |  | [required] |

### Return type

[**models::IssueTypeDetails**](IssueTypeDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_issue_type_avatar

> models::Avatar create_issue_type_avatar(id, size, body, x, y)
Load issue type avatar

Loads an avatar for the issue type.  Specify the avatar's local file location in the body of the request. Also, include the following headers:   *  `X-Atlassian-Token: no-check` To prevent XSRF protection blocking the request, for more information see [Special Headers](#special-request-headers).  *  `Content-Type: image/image type` Valid image types are JPEG, GIF, or PNG.  For example:   `curl --request POST \\ --user email@example.com:<api_token> \\ --header 'X-Atlassian-Token: no-check' \\ --header 'Content-Type: image/< image_type>' \\ --data-binary \"<@/path/to/file/with/your/avatar>\" \\ --url 'https://your-domain.atlassian.net/rest/api/3/issuetype/{issueTypeId}'This`  The avatar is cropped to a square. If no crop parameters are specified, the square originates at the top left of the image. The length of the square's sides is set to the smaller of the height or width of the image.  The cropped image is then used to create avatars of 16x16, 24x24, 32x32, and 48x48 in size.  After creating the avatar, use [ Update issue type](#api-rest-api-3-issuetype-id-put) to set it as the issue type's displayed avatar.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue type. | [required] |
**size** | **i32** | The length of each side of the crop region. | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |
**x** | Option<**i32**> | The X coordinate of the top-left corner of the crop region. |  |[default to 0]
**y** | Option<**i32**> | The Y coordinate of the top-left corner of the crop region. |  |[default to 0]

### Return type

[**models::Avatar**](Avatar.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_issue_type

> delete_issue_type(id, alternative_issue_type_id)
Delete issue type

Deletes the issue type. If the issue type is in use, all uses are updated with the alternative issue type (`alternativeIssueTypeId`). A list of alternative issue types are obtained from the [Get alternative issue types](#api-rest-api-3-issuetype-id-alternatives-get) resource.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue type. | [required] |
**alternative_issue_type_id** | Option<**String**> | The ID of the replacement issue type. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alternative_issue_types

> Vec<models::IssueTypeDetails> get_alternative_issue_types(id)
Get alternative issue types

Returns a list of issue types that can be used to replace the issue type. The alternative issue types are those assigned to the same workflow scheme, field configuration scheme, and screen scheme.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue type. | [required] |

### Return type

[**Vec<models::IssueTypeDetails>**](IssueTypeDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_all_types

> Vec<models::IssueTypeDetails> get_issue_all_types()
Get all issue types for user

Returns all issue types.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Issue types are only returned as follows:   *  if the user has the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), all issue types are returned.  *  if the user has the *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for one or more projects, the issue types associated with the projects the user has permission to browse are returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IssueTypeDetails>**](IssueTypeDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_type

> models::IssueTypeDetails get_issue_type(id)
Get issue type

Returns an issue type.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) in a project the issue type is associated with or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue type. | [required] |

### Return type

[**models::IssueTypeDetails**](IssueTypeDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_types_for_project

> Vec<models::IssueTypeDetails> get_issue_types_for_project(project_id, level)
Get issue types for project

Returns issue types for a project.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) in the relevant project or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i64** | The ID of the project. | [required] |
**level** | Option<**i32**> | The level of the issue type to filter by. Use:   *  `-1` for Subtask.  *  `0` for Base.  *  `1` for Epic. |  |

### Return type

[**Vec<models::IssueTypeDetails>**](IssueTypeDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_issue_type

> models::IssueTypeDetails update_issue_type(id, issue_type_update_bean)
Update issue type

Updates the issue type.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue type. | [required] |
**issue_type_update_bean** | [**IssueTypeUpdateBean**](IssueTypeUpdateBean.md) |  | [required] |

### Return type

[**models::IssueTypeDetails**](IssueTypeDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

