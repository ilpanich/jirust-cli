# WorkflowPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the workflow | [optional]
**looped_transition_container_layout** | Option<[**models::WorkflowStatusLayoutPayload**](WorkflowStatusLayoutPayload.md)> |  | [optional]
**name** | Option<**String**> | The name of the workflow | [optional]
**on_conflict** | Option<**String**> | The strategy to use if there is a conflict with another workflow | [optional][default to New]
**pcri** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]
**start_point_layout** | Option<[**models::WorkflowStatusLayoutPayload**](WorkflowStatusLayoutPayload.md)> |  | [optional]
**statuses** | Option<[**Vec<models::WorkflowStatusPayload>**](WorkflowStatusPayload.md)> | The statuses to be used in the workflow | [optional]
**transitions** | Option<[**Vec<models::TransitionPayload>**](TransitionPayload.md)> | The transitions for the workflow | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


