# WorkflowTransitionRulesUpdateErrorDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rule_update_errors** | [**std::collections::HashMap<String, Vec<String>>**](set.md) | A list of transition rule update errors, indexed by the transition rule ID. Any transition rule that appears here wasn't updated. | 
**update_errors** | **Vec<String>** | The list of errors that specify why the workflow update failed. The workflow was not updated if the list contains any entries. | 
**workflow_id** | [**models::WorkflowId**](WorkflowId.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


