# NotificationRecipients

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assignee** | Option<**bool**> | Whether the notification should be sent to the issue's assignees. | [optional]
**group_ids** | Option<**Vec<String>**> | List of groupIds to receive the notification. | [optional]
**groups** | Option<[**Vec<models::GroupName>**](GroupName.md)> | List of groups to receive the notification. | [optional]
**reporter** | Option<**bool**> | Whether the notification should be sent to the issue's reporter. | [optional]
**users** | Option<[**Vec<models::UserDetails>**](UserDetails.md)> | List of users to receive the notification. | [optional]
**voters** | Option<**bool**> | Whether the notification should be sent to the issue's voters. | [optional]
**watchers** | Option<**bool**> | Whether the notification should be sent to the issue's watchers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


