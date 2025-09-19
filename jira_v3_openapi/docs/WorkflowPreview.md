# WorkflowPreview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the workflow. | [optional]
**id** | Option<**String**> | The ID of the workflow. | [optional]
**looped_transition_container_layout** | Option<[**models::WorkflowPreviewLayout**](WorkflowPreviewLayout.md)> |  | [optional]
**name** | Option<**String**> | The name of the workflow. | [optional]
**query_context** | Option<[**Vec<models::ProjectIssueTypeQueryContext>**](ProjectIssueTypeQueryContext.md)> | The project and issue type context for this workflow query. | [optional]
**scope** | Option<[**models::WorkflowPreviewScope**](WorkflowPreviewScope.md)> |  | [optional]
**start_point_layout** | Option<[**models::WorkflowPreviewLayout**](WorkflowPreviewLayout.md)> |  | [optional]
**statuses** | Option<[**Vec<models::WorkflowPreviewStatus>**](WorkflowPreviewStatus.md)> | The statuses referenced in this workflow. | [optional]
**transitions** | Option<[**Vec<models::TransitionPreview>**](TransitionPreview.md)> | The transitions of the workflow. | [optional]
**version** | Option<[**models::WorkflowDocumentVersionBean**](WorkflowDocumentVersionBean.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


