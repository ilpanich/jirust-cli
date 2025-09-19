# \ProjectTemplatesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_with_custom_template**](ProjectTemplatesApi.md#create_project_with_custom_template) | **POST** /rest/api/3/project-template | Create custom project
[**edit_template**](ProjectTemplatesApi.md#edit_template) | **PUT** /rest/api/3/project-template/edit-template | Edit a custom project template
[**live_template**](ProjectTemplatesApi.md#live_template) | **GET** /rest/api/3/project-template/live-template | Gets a custom project template
[**remove_template**](ProjectTemplatesApi.md#remove_template) | **DELETE** /rest/api/3/project-template/remove-template | Deletes a custom project template
[**save_template**](ProjectTemplatesApi.md#save_template) | **POST** /rest/api/3/project-template/save-template | Save a custom project template



## create_project_with_custom_template

> create_project_with_custom_template(project_custom_template_create_request_dto)
Create custom project

Creates a project based on a custom template provided in the request.  The request body should contain the project details and the capabilities that comprise the project:   *  `details` \\- represents the project details settings  *  `template` \\- represents a list of capabilities responsible for creating specific parts of a project  A capability is defined as a unit of configuration for the project you want to create.  This operation is:   *  [asynchronous](#async). Follow the `Location` link in the response header to determine the status of the task and use [Get task](#api-rest-api-3-task-taskId-get) to obtain subsequent updates.  ***Note: This API is only supported for Jira Enterprise edition.***

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_custom_template_create_request_dto** | [**ProjectCustomTemplateCreateRequestDto**](ProjectCustomTemplateCreateRequestDto.md) | The JSON payload containing the project details and capabilities | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_template

> serde_json::Value edit_template(edit_template_request)
Edit a custom project template

Edit custom template  This API endpoint allows you to edit an existing customised template.  ***Note: Custom Templates are only supported for Jira Enterprise edition.***

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edit_template_request** | [**EditTemplateRequest**](EditTemplateRequest.md) | The object containing the updated template details: name, description | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## live_template

> models::ProjectTemplateModel live_template(project_id, template_key)
Gets a custom project template

Get custom template  This API endpoint allows you to get a live custom project template details by either templateKey or projectId  ***Note: Custom Templates are only supported for Jira Enterprise edition.***

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**String**> | optional - The \\{@link String\\} containing the project key linked to the custom template to retrieve |  |
**template_key** | Option<**String**> | optional - The \\{@link String\\} containing the key of the custom template to retrieve |  |

### Return type

[**models::ProjectTemplateModel**](ProjectTemplateModel.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_template

> serde_json::Value remove_template(template_key)
Deletes a custom project template

Remove custom template  This API endpoint allows you to remove a specified customised template  ***Note: Custom Templates are only supported for Jira Enterprise edition.***

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_key** | **String** | The \\{@link String\\} containing the key of the custom template to remove | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_template

> models::SaveTemplateResponse save_template(save_template_request)
Save a custom project template

Save custom template  This API endpoint allows you to save a customised template  ***Note: Custom Templates are only supported for Jira Enterprise edition.***

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**save_template_request** | [**SaveTemplateRequest**](SaveTemplateRequest.md) | The object containing the template basic details: name, description | [required] |

### Return type

[**models::SaveTemplateResponse**](SaveTemplateResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

