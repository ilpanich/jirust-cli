# TransitionPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions** | Option<[**Vec<models::RulePayload>**](RulePayload.md)> | The actions that are performed when the transition is made | [optional]
**conditions** | Option<[**models::ConditionGroupPayload**](ConditionGroupPayload.md)> |  | [optional]
**custom_issue_event_id** | Option<**String**> | Mechanism in Jira for triggering certain actions, like notifications, automations, etc. Unless a custom notification scheme is configure, it's better not to provide any value here | [optional]
**description** | Option<**String**> | The description of the transition | [optional]
**from** | Option<[**Vec<models::FromLayoutPayload>**](FromLayoutPayload.md)> | The statuses that the transition can be made from | [optional]
**id** | Option<**i32**> | The id of the transition | [optional]
**name** | Option<**String**> | The name of the transition | [optional]
**properties** | Option<**std::collections::HashMap<String, String>**> | The properties of the transition | [optional]
**to** | Option<[**models::ToLayoutPayload**](ToLayoutPayload.md)> |  | [optional]
**transition_screen** | Option<[**models::RulePayload**](RulePayload.md)> |  | [optional]
**triggers** | Option<[**Vec<models::RulePayload>**](RulePayload.md)> | The triggers that are performed when the transition is made | [optional]
**r#type** | Option<**String**> | The type of the transition | [optional]
**validators** | Option<[**Vec<models::RulePayload>**](RulePayload.md)> | The validators that are performed when the transition is made | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


