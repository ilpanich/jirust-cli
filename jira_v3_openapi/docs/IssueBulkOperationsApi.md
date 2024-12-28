# \IssueBulkOperationsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_available_transitions**](IssueBulkOperationsApi.md#get_available_transitions) | **GET** /rest/api/3/bulk/issues/transition | Get available transitions
[**get_bulk_editable_fields**](IssueBulkOperationsApi.md#get_bulk_editable_fields) | **GET** /rest/api/3/bulk/issues/fields | Get bulk editable fields
[**get_bulk_operation_progress**](IssueBulkOperationsApi.md#get_bulk_operation_progress) | **GET** /rest/api/3/bulk/queue/{taskId} | Get bulk issue operation progress
[**submit_bulk_delete**](IssueBulkOperationsApi.md#submit_bulk_delete) | **POST** /rest/api/3/bulk/issues/delete | Bulk delete issues
[**submit_bulk_edit**](IssueBulkOperationsApi.md#submit_bulk_edit) | **POST** /rest/api/3/bulk/issues/fields | Bulk edit issues
[**submit_bulk_move**](IssueBulkOperationsApi.md#submit_bulk_move) | **POST** /rest/api/3/bulk/issues/move | Bulk move issues
[**submit_bulk_transition**](IssueBulkOperationsApi.md#submit_bulk_transition) | **POST** /rest/api/3/bulk/issues/transition | Bulk transition issue statuses



## get_available_transitions

> models::BulkTransitionGetAvailableTransitions get_available_transitions(issue_ids_or_keys, ending_before, starting_after)
Get available transitions

Use this API to retrieve a list of transitions available for the specified issues that can be used or bulk transition operations. You can submit either single or multiple issues in the query to obtain the available transitions.  The response will provide the available transitions for issues, organized by their respective workflows. **Only the transitions that are common among the issues within that workflow and do not involve any additional field updates will be included.** For bulk transitions that require additional field updates, please utilise the Jira Cloud UI.  You can request available transitions for up to 1,000 issues in a single operation. This API uses pagination to return responses, delivering 50 workflows at a time.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Transition [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Transition-issues/) in all projects that contain the selected issues.  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_ids_or_keys** | **String** | Comma (,) separated Ids or keys of the issues to get transitions available for them. | [required] |
**ending_before** | Option<**String**> | (Optional)The end cursor for use in pagination. |  |
**starting_after** | Option<**String**> | (Optional)The start cursor for use in pagination. |  |

### Return type

[**models::BulkTransitionGetAvailableTransitions**](BulkTransitionGetAvailableTransitions.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bulk_editable_fields

> models::BulkEditGetFields get_bulk_editable_fields(issue_ids_or_keys, search_text, ending_before, starting_after)
Get bulk editable fields

Use this API to get a list of fields visible to the user to perform bulk edit operations. You can pass single or multiple issues in the query to get eligible editable fields. This API uses pagination to return responses, delivering 50 fields at a time.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  Depending on the field, any field-specific permissions required to edit it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_ids_or_keys** | **String** | The IDs or keys of the issues to get editable fields from. | [required] |
**search_text** | Option<**String**> | (Optional)The text to search for in the editable fields. |  |
**ending_before** | Option<**String**> | (Optional)The end cursor for use in pagination. |  |
**starting_after** | Option<**String**> | (Optional)The start cursor for use in pagination. |  |

### Return type

[**models::BulkEditGetFields**](BulkEditGetFields.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bulk_operation_progress

> models::BulkOperationProgress get_bulk_operation_progress(task_id)
Get bulk issue operation progress

Use this to get the progress state for the specified bulk operation `taskId`.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Administer Jira [global permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/), or be the creator of the task.  If the task is running, this resource will return:      {\"taskId\":\"10779\",\"status\":\"RUNNING\",\"progressPercent\":65,\"submittedBy\":{\"accountId\":\"5b10a2844c20165700ede21g\"},\"created\":1690180055963,\"started\":1690180056206,\"updated\":169018005829}  If the task has completed, then this resource will return:      {\"processedAccessibleIssues\":[10001,10002],\"created\":1709189449954,\"progressPercent\":100,\"started\":1709189450154,\"status\":\"COMPLETE\",\"submittedBy\":{\"accountId\":\"5b10a2844c20165700ede21g\"},\"invalidOrInaccessibleIssueCount\":0,\"taskId\":\"10000\",\"totalIssueCount\":2,\"updated\":1709189450354}  **Note:** You can view task progress for up to 14 days from creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the task. | [required] |

### Return type

[**models::BulkOperationProgress**](BulkOperationProgress.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_bulk_delete

> models::SubmittedBulkOperation submit_bulk_delete(issue_bulk_delete_payload)
Bulk delete issues

Use this API to submit a bulk delete request. You can delete up to 1,000 issues in a single operation.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Delete [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Delete-issues/) in all projects that contain the selected issues.  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_bulk_delete_payload** | [**IssueBulkDeletePayload**](IssueBulkDeletePayload.md) | The request body containing the issues to be deleted. | [required] |

### Return type

[**models::SubmittedBulkOperation**](SubmittedBulkOperation.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_bulk_edit

> models::SubmittedBulkOperation submit_bulk_edit(issue_bulk_edit_payload)
Bulk edit issues

Use this API to submit a bulk edit request and simultaneously edit multiple issues. There are limits applied to the number of issues and fields that can be edited. A single request can accommodate a maximum of 1000 issues (including subtasks) and 200 fields.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.  *  Edit [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_bulk_edit_payload** | [**IssueBulkEditPayload**](IssueBulkEditPayload.md) | The request body containing the issues to be edited and the new field values. | [required] |

### Return type

[**models::SubmittedBulkOperation**](SubmittedBulkOperation.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_bulk_move

> models::SubmittedBulkOperation submit_bulk_move(issue_bulk_move_payload)
Bulk move issues

Use this API to submit a bulk issue move request. You can move multiple issues, but they must all be moved to and from a single project, issue type, and parent. You can't move more than 1000 issues (including subtasks) at once.  #### Scenarios: ####  This is an early version of the API and it doesn't have full feature parity with the Bulk Move UI experience.   *  Moving issue of type A to issue of type B in the same project or a different project: `SUPPORTED`  *  Moving multiple issues of type A in one project to multiple issues of type B in the same project or a different project: **`SUPPORTED`**  *  Moving a standard parent issue of type A with its multiple subtask issue types in one project to standard issue of type B and multiple subtask issue types in the same project or a different project: `SUPPORTED`  *  Moving an epic issue with its child issues to a different project without losing their relation: `NOT SUPPORTED`       (Workaround: Move them individually and stitch the relationship back with the Bulk Edit API)  #### Limits applied to bulk issue moves: ####  When using the bulk move, keep in mind that there are limits on the number of issues and fields you can include.   *  You can move up to 1,000 issues in a single operation, including any subtasks.  *  All issues must originate from the same project and share the same issue type and parent.  *  The total combined number of fields across all issues must not exceed 1,500,000. For example, if each issue includes 15,000 fields, then the maximum number of issues that can be moved is 100.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Move [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in source projects.  *  Create [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in destination projects.  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in destination projects, if moving subtasks only.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_bulk_move_payload** | [**IssueBulkMovePayload**](IssueBulkMovePayload.md) |  | [required] |

### Return type

[**models::SubmittedBulkOperation**](SubmittedBulkOperation.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_bulk_transition

> models::SubmittedBulkOperation submit_bulk_transition(issue_bulk_transition_payload)
Bulk transition issue statuses

Use this API to submit a bulk issue status transition request. You can transition multiple issues, alongside with their valid transition Ids. You can transition up to 1,000 issues in a single operation.  **[Permissions](#permissions) required:**   *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).  *  Transition [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Transition-issues/) in all projects that contain the selected issues.  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_bulk_transition_payload** | [**IssueBulkTransitionPayload**](IssueBulkTransitionPayload.md) | The request body containing the issues to be transitioned. | [required] |

### Return type

[**models::SubmittedBulkOperation**](SubmittedBulkOperation.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

