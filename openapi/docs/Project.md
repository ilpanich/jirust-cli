# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**archived** | Option<**bool**> | Whether the project is archived. | [optional][readonly]
**archived_by** | Option<[**models::User**](User.md)> | The user who archived the project. | [optional][readonly]
**archived_date** | Option<**String**> | The date when the project was archived. | [optional][readonly]
**assignee_type** | Option<**String**> | The default assignee when creating issues for this project. | [optional][readonly]
**avatar_urls** | Option<[**models::AvatarUrlsBean**](AvatarUrlsBean.md)> | The URLs of the project's avatars. | [optional][readonly]
**components** | Option<[**Vec<models::ProjectComponent>**](ProjectComponent.md)> | List of the components contained in the project. | [optional][readonly]
**deleted** | Option<**bool**> | Whether the project is marked as deleted. | [optional][readonly]
**deleted_by** | Option<[**models::User**](User.md)> | The user who marked the project as deleted. | [optional][readonly]
**deleted_date** | Option<**String**> | The date when the project was marked as deleted. | [optional][readonly]
**description** | Option<**String**> | A brief description of the project. | [optional][readonly]
**email** | Option<**String**> | An email address associated with the project. | [optional]
**expand** | Option<**String**> | Expand options that include additional project details in the response. | [optional][readonly]
**favourite** | Option<**bool**> | Whether the project is selected as a favorite. | [optional]
**id** | Option<**String**> | The ID of the project. | [optional]
**insight** | Option<[**models::ProjectInsight**](ProjectInsight.md)> | Insights about the project. | [optional][readonly]
**is_private** | Option<**bool**> | Whether the project is private from the user's perspective. This means the user can't see the project or any associated issues. | [optional][readonly]
**issue_type_hierarchy** | Option<[**models::Hierarchy**](Hierarchy.md)> | The issue type hierarchy for the project. | [optional][readonly]
**issue_types** | Option<[**Vec<models::IssueTypeDetails>**](IssueTypeDetails.md)> | List of the issue types available in the project. | [optional][readonly]
**key** | Option<**String**> | The key of the project. | [optional][readonly]
**landing_page_info** | Option<[**models::ProjectLandingPageInfo**](ProjectLandingPageInfo.md)> | The project landing page info. | [optional][readonly]
**lead** | Option<[**models::User**](User.md)> | The username of the project lead. | [optional][readonly]
**name** | Option<**String**> | The name of the project. | [optional][readonly]
**permissions** | Option<[**models::ProjectPermissions**](ProjectPermissions.md)> | User permissions on the project | [optional][readonly]
**project_category** | Option<[**models::ProjectCategory**](ProjectCategory.md)> | The category the project belongs to. | [optional][readonly]
**project_type_key** | Option<**String**> | The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes) of the project. | [optional][readonly]
**properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Map of project properties | [optional][readonly]
**retention_till_date** | Option<**String**> | The date when the project is deleted permanently. | [optional][readonly]
**roles** | Option<**std::collections::HashMap<String, String>**> | The name and self URL for each role defined in the project. For more information, see [Create project role](#api-rest-api-3-role-post). | [optional][readonly]
**param_self** | Option<**String**> | The URL of the project details. | [optional][readonly]
**simplified** | Option<**bool**> | Whether the project is simplified. | [optional][readonly]
**style** | Option<**String**> | The type of the project. | [optional][readonly]
**url** | Option<**String**> | A link to information about this project, such as project documentation. | [optional][readonly]
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique ID for next-gen projects. | [optional][readonly]
**versions** | Option<[**Vec<models::Version>**](Version.md)> | The versions defined in the project. For more information, see [Create version](#api-rest-api-3-version-post). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


