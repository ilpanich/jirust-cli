# Workflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | The creation date of the workflow. | [optional]
**description** | **String** | The description of the workflow. | 
**has_draft_workflow** | Option<**bool**> | Whether the workflow has a draft version. | [optional]
**id** | [**models::PublishedWorkflowId**](PublishedWorkflowId.md) |  | 
**is_default** | Option<**bool**> | Whether this is the default workflow. | [optional]
**operations** | Option<[**models::WorkflowOperations**](WorkflowOperations.md)> |  | [optional]
**projects** | Option<[**Vec<models::ProjectDetails>**](ProjectDetails.md)> | The projects the workflow is assigned to, through workflow schemes. | [optional]
**schemes** | Option<[**Vec<models::WorkflowSchemeIdName>**](WorkflowSchemeIdName.md)> | The workflow schemes the workflow is assigned to. | [optional]
**statuses** | Option<[**Vec<models::WorkflowStatus>**](WorkflowStatus.md)> | The statuses of the workflow. | [optional]
**transitions** | Option<[**Vec<models::Transition>**](Transition.md)> | The transitions of the workflow. | [optional]
**updated** | Option<**String**> | The last edited date of the workflow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


