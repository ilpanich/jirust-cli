# NewUserDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_keys** | Option<**Vec<String>**> | Deprecated, do not use. | [optional]
**display_name** | Option<**String**> | This property is no longer available. If the user has an Atlassian account, their display name is not changed. If the user does not have an Atlassian account, they are sent an email asking them set up an account. | [optional]
**email_address** | **String** | The email address for the user. | 
**key** | Option<**String**> | This property is no longer available. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. | [optional]
**name** | Option<**String**> | This property is no longer available. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. | [optional]
**password** | Option<**String**> | This property is no longer available. If the user has an Atlassian account, their password is not changed. If the user does not have an Atlassian account, they are sent an email asking them set up an account. | [optional]
**products** | **Vec<String>** | Products the new user has access to. Valid products are: jira-core, jira-servicedesk, jira-product-discovery, jira-software. To create a user without product access, set this field to be an empty array. | 
**param_self** | Option<**String**> | The URL of the user. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


