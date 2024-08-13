# EventNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_address** | Option<**String**> | The email address. | [optional]
**expand** | Option<**String**> | Expand options that include additional event notification details in the response. | [optional]
**field** | Option<[**models::FieldDetails**](FieldDetails.md)> | The custom user or group field. | [optional]
**group** | Option<[**models::GroupName**](GroupName.md)> | The specified group. | [optional]
**id** | Option<**i64**> | The ID of the notification. | [optional]
**notification_type** | Option<**String**> | Identifies the recipients of the notification. | [optional]
**parameter** | Option<**String**> | As a group's name can change, use of `recipient` is recommended. The identifier associated with the `notificationType` value that defines the receiver of the notification, where the receiver isn't implied by `notificationType` value. So, when `notificationType` is:   *  `User` The `parameter` is the user account ID.  *  `Group` The `parameter` is the group name.  *  `ProjectRole` The `parameter` is the project role ID.  *  `UserCustomField` The `parameter` is the ID of the custom field.  *  `GroupCustomField` The `parameter` is the ID of the custom field. | [optional]
**project_role** | Option<[**models::ProjectRole**](ProjectRole.md)> | The specified project role. | [optional]
**recipient** | Option<**String**> | The identifier associated with the `notificationType` value that defines the receiver of the notification, where the receiver isn't implied by the `notificationType` value. So, when `notificationType` is:   *  `User`, `recipient` is the user account ID.  *  `Group`, `recipient` is the group ID.  *  `ProjectRole`, `recipient` is the project role ID.  *  `UserCustomField`, `recipient` is the ID of the custom field.  *  `GroupCustomField`, `recipient` is the ID of the custom field. | [optional]
**user** | Option<[**models::UserDetails**](UserDetails.md)> | The specified user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


