# Comment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | Option<[**models::UserDetails**](UserDetails.md)> | The ID of the user who created the comment. | [optional][readonly]
**body** | Option<[**serde_json::Value**](.md)> | The comment text in [Atlassian Document Format](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure/). | [optional]
**created** | Option<**String**> | The date and time at which the comment was created. | [optional][readonly]
**id** | Option<**String**> | The ID of the comment. | [optional][readonly]
**jsd_author_can_see_request** | Option<**bool**> | Whether the comment was added from an email sent by a person who is not part of the issue. See [Allow external emails to be added as comments on issues](https://support.atlassian.com/jira-service-management-cloud/docs/allow-external-emails-to-be-added-as-comments-on-issues/)for information on setting up this feature. | [optional][readonly]
**jsd_public** | Option<**bool**> | Whether the comment is visible in Jira Service Desk. Defaults to true when comments are created in the Jira Cloud Platform. This includes when the site doesn't use Jira Service Desk or the project isn't a Jira Service Desk project and, therefore, there is no Jira Service Desk for the issue to be visible on. To create a comment with its visibility in Jira Service Desk set to false, use the Jira Service Desk REST API [Create request comment](https://developer.atlassian.com/cloud/jira/service-desk/rest/#api-rest-servicedeskapi-request-issueIdOrKey-comment-post) operation. | [optional][readonly]
**properties** | Option<[**Vec<models::EntityProperty>**](EntityProperty.md)> | A list of comment properties. Optional on create and update. | [optional]
**rendered_body** | Option<**String**> | The rendered version of the comment. | [optional][readonly]
**param_self** | Option<**String**> | The URL of the comment. | [optional][readonly]
**update_author** | Option<[**models::UserDetails**](UserDetails.md)> | The ID of the user who updated the comment last. | [optional][readonly]
**updated** | Option<**String**> | The date and time at which the comment was updated last. | [optional][readonly]
**visibility** | Option<[**models::Visibility**](Visibility.md)> | The group or role to which this comment is visible. Optional on create and update. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


