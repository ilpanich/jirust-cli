# ConditionGroupUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition_groups** | Option<[**Vec<models::ConditionGroupUpdate>**](ConditionGroupUpdate.md)> | The nested conditions of the condition group. | [optional]
**conditions** | Option<[**Vec<models::WorkflowRuleConfiguration>**](WorkflowRuleConfiguration.md)> | The rules for this condition. | [optional]
**operation** | **String** | Determines how the conditions in the group are evaluated. Accepts either `ANY` or `ALL`. If `ANY` is used, at least one condition in the group must be true for the group to evaluate to true. If `ALL` is used, all conditions in the group must be true for the group to evaluate to true. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


