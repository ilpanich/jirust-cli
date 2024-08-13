# UserDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. | [optional]
**account_type** | Option<**String**> | The type of account represented by this user. This will be one of 'atlassian' (normal users), 'app' (application user) or 'customer' (Jira Service Desk customer user) | [optional][readonly]
**active** | Option<**bool**> | Whether the user is active. | [optional][readonly]
**avatar_urls** | Option<[**models::AvatarUrlsBean**](AvatarUrlsBean.md)> | The avatars of the user. | [optional][readonly]
**display_name** | Option<**String**> | The display name of the user. Depending on the user’s privacy settings, this may return an alternative value. | [optional][readonly]
**email_address** | Option<**String**> | The email address of the user. Depending on the user’s privacy settings, this may be returned as null. | [optional][readonly]
**key** | Option<**String**> | This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. | [optional][readonly]
**name** | Option<**String**> | This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. | [optional][readonly]
**param_self** | Option<**String**> | The URL of the user. | [optional][readonly]
**time_zone** | Option<**String**> | The time zone specified in the user's profile. Depending on the user’s privacy settings, this may be returned as null. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


