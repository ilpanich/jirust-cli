# UserBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. | [optional]
**active** | Option<**bool**> | Whether the user is active. | [optional]
**avatar_urls** | Option<[**models::UserBeanAvatarUrls**](UserBeanAvatarUrls.md)> | The avatars of the user. | [optional]
**display_name** | Option<**String**> | The display name of the user. Depending on the user’s privacy setting, this may return an alternative value. | [optional]
**key** | Option<**String**> | This property is deprecated in favor of `accountId` because of privacy changes. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.   The key of the user. | [optional]
**name** | Option<**String**> | This property is deprecated in favor of `accountId` because of privacy changes. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.   The username of the user. | [optional]
**param_self** | Option<**String**> | The URL of the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


