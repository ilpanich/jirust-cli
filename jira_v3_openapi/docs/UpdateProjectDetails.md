# UpdateProjectDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assignee_type** | Option<**String**> | The default assignee when creating issues for this project. | [optional]
**avatar_id** | Option<**i64**> | An integer value for the project's avatar. | [optional]
**category_id** | Option<**i64**> | The ID of the project's category. A complete list of category IDs is found using the [Get all project categories](#api-rest-api-3-projectCategory-get) operation. To remove the project category from the project, set the value to `-1.` | [optional]
**description** | Option<**String**> | A brief description of the project. | [optional]
**issue_security_scheme** | Option<**i64**> | The ID of the issue security scheme for the project, which enables you to control who can and cannot view issues. Use the [Get issue security schemes](#api-rest-api-3-issuesecurityschemes-get) resource to get all issue security scheme IDs. | [optional]
**key** | Option<**String**> | Project keys must be unique and start with an uppercase letter followed by one or more uppercase alphanumeric characters. The maximum length is 10 characters. | [optional]
**lead** | Option<**String**> | This parameter is deprecated because of privacy changes. Use `leadAccountId` instead. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. The user name of the project lead. Cannot be provided with `leadAccountId`. | [optional]
**lead_account_id** | Option<**String**> | The account ID of the project lead. Cannot be provided with `lead`. | [optional]
**name** | Option<**String**> | The name of the project. | [optional]
**notification_scheme** | Option<**i64**> | The ID of the notification scheme for the project. Use the [Get notification schemes](#api-rest-api-3-notificationscheme-get) resource to get a list of notification scheme IDs. | [optional]
**permission_scheme** | Option<**i64**> | The ID of the permission scheme for the project. Use the [Get all permission schemes](#api-rest-api-3-permissionscheme-get) resource to see a list of all permission scheme IDs. | [optional]
**released_project_keys** | Option<**Vec<String>**> | Previous project keys to be released from the current project. Released keys must belong to the current project and not contain the current project key | [optional]
**url** | Option<**String**> | A link to information about this project, such as project documentation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


