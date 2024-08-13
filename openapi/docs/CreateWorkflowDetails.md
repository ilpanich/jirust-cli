# CreateWorkflowDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the workflow. The maximum length is 1000 characters. | [optional]
**name** | **String** | The name of the workflow. The name must be unique. The maximum length is 255 characters. Characters can be separated by a whitespace but the name cannot start or end with a whitespace. | 
**statuses** | [**Vec<models::CreateWorkflowStatusDetails>**](CreateWorkflowStatusDetails.md) | The statuses of the workflow. Any status that does not include a transition is added to the workflow without a transition. | 
**transitions** | [**Vec<models::CreateWorkflowTransitionDetails>**](CreateWorkflowTransitionDetails.md) | The transitions of the workflow. For the request to be valid, these transitions must:   *  include one *initial* transition.  *  not use the same name for a *global* and *directed* transition.  *  have a unique name for each *global* transition.  *  have a unique 'to' status for each *global* transition.  *  have unique names for each transition from a status.  *  not have a 'from' status on *initial* and *global* transitions.  *  have a 'from' status on *directed* transitions.  All the transition statuses must be included in `statuses`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


