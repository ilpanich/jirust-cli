# UiModificationContextDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the UI modification context. | [optional][readonly]
**is_available** | Option<**bool**> | Whether a context is available. For example, when a project is deleted the context becomes unavailable. | [optional][readonly]
**issue_type_id** | Option<**String**> | The issue type ID of the context. Null is treated as a wildcard, meaning the UI modification will be applied to all issue types. Each UI modification context can have a maximum of one wildcard. | [optional]
**project_id** | Option<**String**> | The project ID of the context. Null is treated as a wildcard, meaning the UI modification will be applied to all projects. Each UI modification context can have a maximum of one wildcard. | [optional]
**view_type** | Option<**String**> | The view type of the context. Only `GIC`(Global Issue Create), `IssueView` and `IssueTransition` are supported. Null is treated as a wildcard, meaning the UI modification will be applied to all view types. Each UI modification context can have a maximum of one wildcard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


