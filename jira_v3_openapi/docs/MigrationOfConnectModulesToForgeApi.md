# \MigrationOfConnectModulesToForgeApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connect_to_forge_migration_fetch_task_resource_period_fetch_migration_task_get**](MigrationOfConnectModulesToForgeApi.md#connect_to_forge_migration_fetch_task_resource_period_fetch_migration_task_get) | **GET** /rest/atlassian-connect/1/migration/{connectKey}/{jiraIssueFieldsKey}/task | Get Connect issue field migration task



## connect_to_forge_migration_fetch_task_resource_period_fetch_migration_task_get

> models::TaskProgress connect_to_forge_migration_fetch_task_resource_period_fetch_migration_task_get(connect_key, jira_issue_fields_key)
Get Connect issue field migration task

Returns the details of a Connect issue field's migration to Forge.  When migrating a Connect app to Forge, [Issue Field](https://developer.atlassian.com/cloud/jira/software/modules/issue-field/) modules must be converted to [Custom field](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/). When the Forge version of the app is installed, Forge creates a [background task](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-tasks/#api-group-tasks) to track the migration of field data across. This endpoint returns the status and other details of that background task.  For more details, see [Jira modules > Jira Custom Fields](https://developer.atlassian.com/platform/adopting-forge-from-connect/migrate-jira-custom-fields/).  **[Permissions](#permissions) required:** Only Connect and Forge apps can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connect_key** | **String** | The key of the Connect app that contains the Jira issue field being migrated. | [required] |
**jira_issue_fields_key** | **String** | The module key of the Connect issue field being migrated. | [required] |

### Return type

[**models::TaskProgress**](TaskProgress.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

