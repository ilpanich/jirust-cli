# NotificationEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the event. | [optional]
**id** | Option<**i64**> | The ID of the event. The event can be a [Jira system event](https://confluence.atlassian.com/x/8YdKLg#Creatinganotificationscheme-eventsEvents) or a [custom event](https://confluence.atlassian.com/x/AIlKLg). | [optional]
**name** | Option<**String**> | The name of the event. | [optional]
**template_event** | Option<[**models::NotificationEvent**](NotificationEvent.md)> | The template of the event. Only custom events configured by Jira administrators have template. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


