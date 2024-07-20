# WorkflowSchemeUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_workflow_id** | Option<**String**> | The ID of the workflow for issue types without having a mapping defined in this workflow scheme. Only used in global-scoped workflow schemes. If the `defaultWorkflowId` isn't specified, this is set to *Jira Workflow (jira)*. | [optional]
**description** | **String** | The new description for this workflow scheme. | 
**id** | **String** | The ID of this workflow scheme. | 
**name** | **String** | The new name for this workflow scheme. | 
**status_mappings_by_issue_type_override** | Option<[**Vec<models::MappingsByIssueTypeOverride>**](MappingsByIssueTypeOverride.md)> | Overrides, for the selected issue types, any status mappings provided in `statusMappingsByWorkflows`. Status mappings are required when the new workflow for an issue type doesn't contain all statuses that the old workflow has. Status mappings can be provided by a combination of `statusMappingsByWorkflows` and `statusMappingsByIssueTypeOverride`. | [optional]
**status_mappings_by_workflows** | Option<[**Vec<models::MappingsByWorkflow>**](MappingsByWorkflow.md)> | The status mappings by workflows. Status mappings are required when the new workflow for an issue type doesn't contain all statuses that the old workflow has. Status mappings can be provided by a combination of `statusMappingsByWorkflows` and `statusMappingsByIssueTypeOverride`. | [optional]
**version** | [**models::DocumentVersion**](DocumentVersion.md) |  | 
**workflows_for_issue_types** | Option<[**Vec<models::WorkflowSchemeAssociation>**](WorkflowSchemeAssociation.md)> | Mappings from workflows to issue types. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


