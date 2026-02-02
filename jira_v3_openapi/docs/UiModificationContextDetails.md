# UiModificationContextDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the UI modification context. | [optional][readonly]
**is_available** | Option<**bool**> | Whether a context is available. For example, when a project is deleted the context becomes unavailable. | [optional][readonly]
**issue_type_id** | Option<**String**> | The issue type ID of the context. Null is treated as a wildcard, meaning the UI modification will be applied to all issue types. Each UI modification context can have a maximum of one wildcard. | [optional]
**portal_id** | Option<**String**> | The portal ID of the context. Only required for Jira Service Management request create portal view (`JSMRequestCreate`). | [optional]
**project_id** | Option<**String**> | The project ID of the context. Null is treated as a wildcard, meaning the UI modification will be applied to all projects. Each UI modification context can have a maximum of one wildcard. | [optional]
**request_type_id** | Option<**String**> | The request type ID of the context. Only required for Jira Service Management request create portal view (`JSMRequestCreate`). | [optional]
**view_type** | Option<**String**> | The view type of the context.   Supported values:   *  `GIC` \\- Jira global issue create  *  `IssueView` \\- Jira issue view  *  `IssueTransition` \\- Jira issue transition  *  `JSMRequestCreate` \\- Jira Service Management request create portal view  For Jira view types (`GIC`, `IssueView`, `IssueTransition`), null is treated as a wildcard, meaning the UI modification will be applied to all view types. Each Jira context can have a maximum of one wildcard.      Wildcards are not applicable for JSM contexts. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


