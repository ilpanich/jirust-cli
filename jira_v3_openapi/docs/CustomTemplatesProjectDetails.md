# CustomTemplatesProjectDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_level** | Option<**String**> | The access level of the project. Only used by team-managed project | [optional]
**additional_properties** | Option<**std::collections::HashMap<String, String>**> | Additional properties of the project | [optional]
**assignee_type** | Option<**String**> | The default assignee when creating issues in the project | [optional]
**avatar_id** | Option<**i64**> | The ID of the project's avatar. Use the \\[Get project avatars\\](\\#api-rest-api-3-project-projectIdOrKey-avatar-get) operation to list the available avatars in a project. | [optional]
**category_id** | Option<**i64**> | The ID of the project's category. A complete list of category IDs is found using the [Get all project categories](#api-rest-api-3-projectCategory-get) operation. | [optional]
**description** | Option<**String**> | Brief description of the project | [optional]
**enable_components** | Option<**bool**> | Whether components are enabled for the project. Only used by company-managed project | [optional][default to false]
**key** | Option<**String**> | Project keys must be unique and start with an uppercase letter followed by one or more uppercase alphanumeric characters. The maximum length is 10 characters. | [optional]
**language** | Option<**String**> | The default language for the project | [optional]
**lead_account_id** | Option<**String**> | The account ID of the project lead. Either `lead` or `leadAccountId` must be set when creating a project. Cannot be provided with `lead`. | [optional]
**name** | Option<**String**> | Name of the project | [optional]
**url** | Option<**String**> | A link to information about this project, such as project documentation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


