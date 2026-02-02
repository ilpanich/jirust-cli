# WorkflowSchemeProjectSwitchBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mappings_by_issue_type_override** | Option<[**Vec<models::MappingsByIssueTypeOverride>**](MappingsByIssueTypeOverride.md)> | The mappings for migrating issues from old statuses to new statuses when switching from one workflow scheme to another. This field is required if any statuses in the current project's workflows would no longer exist in the target workflow scheme. Each mapping defines how to update issues from an old status to the corresponding new status in the issue’s new workflow. | [optional]
**project_id** | Option<**String**> | The ID of the project to switch the workflow scheme for | [optional]
**target_scheme_id** | Option<**String**> | The ID of the target workflow scheme to switch to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


