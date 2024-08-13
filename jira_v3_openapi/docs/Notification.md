# Notification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**html_body** | Option<**String**> | The HTML body of the email notification for the issue. | [optional]
**restrict** | Option<[**models::NotificationRecipientsRestrictions**](NotificationRecipientsRestrictions.md)> | Restricts the notifications to users with the specified permissions. | [optional]
**subject** | Option<**String**> | The subject of the email notification for the issue. If this is not specified, then the subject is set to the issue key and summary. | [optional]
**text_body** | Option<**String**> | The plain text body of the email notification for the issue. | [optional]
**to** | Option<[**models::NotificationRecipients**](NotificationRecipients.md)> | The recipients of the email notification for the issue. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


