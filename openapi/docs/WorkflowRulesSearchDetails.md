# WorkflowRulesSearchDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invalid_rules** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of workflow rule IDs that do not belong to the workflow or can not be found. | [optional]
**valid_rules** | Option<[**Vec<models::WorkflowTransitionRules>**](WorkflowTransitionRules.md)> | List of valid workflow transition rules. | [optional]
**workflow_entity_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The workflow ID. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


