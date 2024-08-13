# WorkflowSchemeUpdateRequiredMappingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status_mappings_by_issue_types** | Option<[**Vec<models::RequiredMappingByIssueType>**](RequiredMappingByIssueType.md)> | The list of required status mappings by issue type. | [optional]
**status_mappings_by_workflows** | Option<[**Vec<models::RequiredMappingByWorkflows>**](RequiredMappingByWorkflows.md)> | The list of required status mappings by workflow. | [optional]
**statuses** | Option<[**Vec<models::StatusMetadata>**](StatusMetadata.md)> | The details of the statuses in the associated workflows. | [optional]
**statuses_per_workflow** | Option<[**Vec<models::StatusesPerWorkflow>**](StatusesPerWorkflow.md)> | The statuses associated with each workflow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


