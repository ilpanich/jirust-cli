# WorkflowScheme

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_workflow** | Option<**String**> | The name of the default workflow for the workflow scheme. The default workflow has *All Unassigned Issue Types* assigned to it in Jira. If `defaultWorkflow` is not specified when creating a workflow scheme, it is set to *Jira Workflow (jira)*. | [optional]
**description** | Option<**String**> | The description of the workflow scheme. | [optional]
**draft** | Option<**bool**> | Whether the workflow scheme is a draft or not. | [optional][readonly]
**id** | Option<**i64**> | The ID of the workflow scheme. | [optional][readonly]
**issue_type_mappings** | Option<**std::collections::HashMap<String, String>**> | The issue type to workflow mappings, where each mapping is an issue type ID and workflow name pair. Note that an issue type can only be mapped to one workflow in a workflow scheme. | [optional]
**issue_types** | Option<[**std::collections::HashMap<String, models::IssueTypeDetails>**](IssueTypeDetails.md)> | The issue types available in Jira. | [optional][readonly]
**last_modified** | Option<**String**> | The date-time that the draft workflow scheme was last modified. A modification is a change to the issue type-project mappings only. This property does not apply to non-draft workflows. | [optional][readonly]
**last_modified_user** | Option<[**models::User**](User.md)> | The user that last modified the draft workflow scheme. A modification is a change to the issue type-project mappings only. This property does not apply to non-draft workflows. | [optional][readonly]
**name** | Option<**String**> | The name of the workflow scheme. The name must be unique. The maximum length is 255 characters. Required when creating a workflow scheme. | [optional]
**original_default_workflow** | Option<**String**> | For draft workflow schemes, this property is the name of the default workflow for the original workflow scheme. The default workflow has *All Unassigned Issue Types* assigned to it in Jira. | [optional][readonly]
**original_issue_type_mappings** | Option<**std::collections::HashMap<String, String>**> | For draft workflow schemes, this property is the issue type to workflow mappings for the original workflow scheme, where each mapping is an issue type ID and workflow name pair. Note that an issue type can only be mapped to one workflow in a workflow scheme. | [optional][readonly]
**param_self** | Option<**String**> |  | [optional][readonly]
**update_draft_if_needed** | Option<**bool**> | Whether to create or update a draft workflow scheme when updating an active workflow scheme. An active workflow scheme is a workflow scheme that is used by at least one project. The following examples show how this property works:   *  Update an active workflow scheme with `updateDraftIfNeeded` set to `true`: If a draft workflow scheme exists, it is updated. Otherwise, a draft workflow scheme is created.  *  Update an active workflow scheme with `updateDraftIfNeeded` set to `false`: An error is returned, as active workflow schemes cannot be updated.  *  Update an inactive workflow scheme with `updateDraftIfNeeded` set to `true`: The workflow scheme is updated, as inactive workflow schemes do not require drafts to update.  Defaults to `false`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


