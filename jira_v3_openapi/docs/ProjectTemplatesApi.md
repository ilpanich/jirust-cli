# \ProjectTemplatesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_with_custom_template**](ProjectTemplatesApi.md#create_project_with_custom_template) | **POST** /rest/api/3/project-template | Create custom project



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

