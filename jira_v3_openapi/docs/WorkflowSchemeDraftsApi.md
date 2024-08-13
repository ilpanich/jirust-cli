# \WorkflowSchemeDraftsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workflow_scheme_draft_from_parent**](WorkflowSchemeDraftsApi.md#create_workflow_scheme_draft_from_parent) | **POST** /rest/api/3/workflowscheme/{id}/createdraft | Create draft workflow scheme
[**delete_draft_default_workflow**](WorkflowSchemeDraftsApi.md#delete_draft_default_workflow) | **DELETE** /rest/api/3/workflowscheme/{id}/draft/default | Delete draft default workflow
[**delete_draft_workflow_mapping**](WorkflowSchemeDraftsApi.md#delete_draft_workflow_mapping) | **DELETE** /rest/api/3/workflowscheme/{id}/draft/workflow | Delete issue types for workflow in draft workflow scheme
[**delete_workflow_scheme_draft**](WorkflowSchemeDraftsApi.md#delete_workflow_scheme_draft) | **DELETE** /rest/api/3/workflowscheme/{id}/draft | Delete draft workflow scheme
[**delete_workflow_scheme_draft_issue_type**](WorkflowSchemeDraftsApi.md#delete_workflow_scheme_draft_issue_type) | **DELETE** /rest/api/3/workflowscheme/{id}/draft/issuetype/{issueType} | Delete workflow for issue type in draft workflow scheme
[**get_draft_default_workflow**](WorkflowSchemeDraftsApi.md#get_draft_default_workflow) | **GET** /rest/api/3/workflowscheme/{id}/draft/default | Get draft default workflow
[**get_draft_workflow**](WorkflowSchemeDraftsApi.md#get_draft_workflow) | **GET** /rest/api/3/workflowscheme/{id}/draft/workflow | Get issue types for workflows in draft workflow scheme
[**get_workflow_scheme_draft**](WorkflowSchemeDraftsApi.md#get_workflow_scheme_draft) | **GET** /rest/api/3/workflowscheme/{id}/draft | Get draft workflow scheme
[**get_workflow_scheme_draft_issue_type**](WorkflowSchemeDraftsApi.md#get_workflow_scheme_draft_issue_type) | **GET** /rest/api/3/workflowscheme/{id}/draft/issuetype/{issueType} | Get workflow for issue type in draft workflow scheme
[**publish_draft_workflow_scheme**](WorkflowSchemeDraftsApi.md#publish_draft_workflow_scheme) | **POST** /rest/api/3/workflowscheme/{id}/draft/publish | Publish draft workflow scheme
[**set_workflow_scheme_draft_issue_type**](WorkflowSchemeDraftsApi.md#set_workflow_scheme_draft_issue_type) | **PUT** /rest/api/3/workflowscheme/{id}/draft/issuetype/{issueType} | Set workflow for issue type in draft workflow scheme
[**update_draft_default_workflow**](WorkflowSchemeDraftsApi.md#update_draft_default_workflow) | **PUT** /rest/api/3/workflowscheme/{id}/draft/default | Update draft default workflow
[**update_draft_workflow_mapping**](WorkflowSchemeDraftsApi.md#update_draft_workflow_mapping) | **PUT** /rest/api/3/workflowscheme/{id}/draft/workflow | Set issue types for workflow in workflow scheme
[**update_workflow_scheme_draft**](WorkflowSchemeDraftsApi.md#update_workflow_scheme_draft) | **PUT** /rest/api/3/workflowscheme/{id}/draft | Update draft workflow scheme



## create_workflow_scheme_draft_from_parent

> models::WorkflowScheme create_workflow_scheme_draft_from_parent(id)
Create draft workflow scheme

Create a draft workflow scheme from an active workflow scheme, by copying the active workflow scheme. Note that an active workflow scheme can only have one draft workflow scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the active workflow scheme that the draft is created from. | [required] |

### Return type

[**models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_draft_default_workflow

> models::WorkflowScheme delete_draft_default_workflow(id)
Delete draft default workflow

Resets the default workflow for a workflow scheme's draft. That is, the default workflow is set to Jira's system workflow (the *jira* workflow).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |

### Return type

[**models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_draft_workflow_mapping

> delete_draft_workflow_mapping(id, workflow_name)
Delete issue types for workflow in draft workflow scheme

Deletes the workflow-issue type mapping for a workflow in a workflow scheme's draft.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |
**workflow_name** | **String** | The name of the workflow. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow_scheme_draft

> delete_workflow_scheme_draft(id)
Delete draft workflow scheme

Deletes a draft workflow scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the active workflow scheme that the draft was created from. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow_scheme_draft_issue_type

> models::WorkflowScheme delete_workflow_scheme_draft_issue_type(id, issue_type)
Delete workflow for issue type in draft workflow scheme

Deletes the issue type-workflow mapping for an issue type in a workflow scheme's draft.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |
**issue_type** | **String** | The ID of the issue type. | [required] |

### Return type

[**models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_draft_default_workflow

> models::DefaultWorkflow get_draft_default_workflow(id)
Get draft default workflow

Returns the default workflow for a workflow scheme's draft. The default workflow is the workflow that is assigned any issue types that have not been mapped to any other workflow. The default workflow has *All Unassigned Issue Types* listed in its issue types for the workflow scheme in Jira.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |

### Return type

[**models::DefaultWorkflow**](DefaultWorkflow.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_draft_workflow

> models::IssueTypesWorkflowMapping get_draft_workflow(id, workflow_name)
Get issue types for workflows in draft workflow scheme

Returns the workflow-issue type mappings for a workflow scheme's draft.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |
**workflow_name** | Option<**String**> | The name of a workflow in the scheme. Limits the results to the workflow-issue type mapping for the specified workflow. |  |

### Return type

[**models::IssueTypesWorkflowMapping**](IssueTypesWorkflowMapping.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_scheme_draft

> models::WorkflowScheme get_workflow_scheme_draft(id)
Get draft workflow scheme

Returns the draft workflow scheme for an active workflow scheme. Draft workflow schemes allow changes to be made to the active workflow schemes: When an active workflow scheme is updated, a draft copy is created. The draft is modified, then the changes in the draft are copied back to the active workflow scheme. See [Configuring workflow schemes](https://confluence.atlassian.com/x/tohKLg) for more information.   Note that:   *  Only active workflow schemes can have draft workflow schemes.  *  An active workflow scheme can only have one draft workflow scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the active workflow scheme that the draft was created from. | [required] |

### Return type

[**models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_scheme_draft_issue_type

> models::IssueTypeWorkflowMapping get_workflow_scheme_draft_issue_type(id, issue_type)
Get workflow for issue type in draft workflow scheme

Returns the issue type-workflow mapping for an issue type in a workflow scheme's draft.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |
**issue_type** | **String** | The ID of the issue type. | [required] |

### Return type

[**models::IssueTypeWorkflowMapping**](IssueTypeWorkflowMapping.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_draft_workflow_scheme

> publish_draft_workflow_scheme(id, publish_draft_workflow_scheme, validate_only)
Publish draft workflow scheme

Publishes a draft workflow scheme.  Where the draft workflow includes new workflow statuses for an issue type, mappings are provided to update issues with the original workflow status to the new workflow status.  This operation is [asynchronous](#async). Follow the `location` link in the response to determine the status of the task and use [Get task](#api-rest-api-3-task-taskId-get) to obtain updates.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |
**publish_draft_workflow_scheme** | [**PublishDraftWorkflowScheme**](PublishDraftWorkflowScheme.md) | Details of the status mappings. | [required] |
**validate_only** | Option<**bool**> | Whether the request only performs a validation. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_workflow_scheme_draft_issue_type

> models::WorkflowScheme set_workflow_scheme_draft_issue_type(id, issue_type, issue_type_workflow_mapping)
Set workflow for issue type in draft workflow scheme

Sets the workflow for an issue type in a workflow scheme's draft.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |
**issue_type** | **String** | The ID of the issue type. | [required] |
**issue_type_workflow_mapping** | [**IssueTypeWorkflowMapping**](IssueTypeWorkflowMapping.md) | The issue type-project mapping. | [required] |

### Return type

[**models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_draft_default_workflow

> models::WorkflowScheme update_draft_default_workflow(id, default_workflow)
Update draft default workflow

Sets the default workflow for a workflow scheme's draft.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |
**default_workflow** | [**DefaultWorkflow**](DefaultWorkflow.md) | The object for the new default workflow. | [required] |

### Return type

[**models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_draft_workflow_mapping

> models::WorkflowScheme update_draft_workflow_mapping(id, workflow_name, issue_types_workflow_mapping)
Set issue types for workflow in workflow scheme

Sets the issue types for a workflow in a workflow scheme's draft. The workflow can also be set as the default workflow for the draft workflow scheme. Unmapped issues types are mapped to the default workflow.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme that the draft belongs to. | [required] |
**workflow_name** | **String** | The name of the workflow. | [required] |
**issue_types_workflow_mapping** | [**IssueTypesWorkflowMapping**](IssueTypesWorkflowMapping.md) |  | [required] |

### Return type

[**models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow_scheme_draft

> models::WorkflowScheme update_workflow_scheme_draft(id, workflow_scheme)
Update draft workflow scheme

Updates a draft workflow scheme. If a draft workflow scheme does not exist for the active workflow scheme, then a draft is created. Note that an active workflow scheme can only have one draft workflow scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the active workflow scheme that the draft was created from. | [required] |
**workflow_scheme** | [**WorkflowScheme**](WorkflowScheme.md) |  | [required] |

### Return type

[**models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

