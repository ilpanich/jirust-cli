# Version

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approvers** | Option<[**Vec<models::VersionApprover>**](VersionApprover.md)> | If the expand option `approvers` is used, returns a list containing the approvers for this version. | [optional][readonly]
**archived** | Option<**bool**> | Indicates that the version is archived. Optional when creating or updating a version. | [optional]
**description** | Option<**String**> | The description of the version. Optional when creating or updating a version. The maximum size is 16,384 bytes. | [optional]
**driver** | Option<**String**> | If the expand option `driver` is used, returns the Atlassian account ID of the driver. | [optional][readonly]
**expand** | Option<**String**> | Use [expand](em>#expansion) to include additional information about version in the response. This parameter accepts a comma-separated list. Expand options include:   *  `operations` Returns the list of operations available for this version.  *  `issuesstatus` Returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property contains a count of issues with a status other than *to do*, *in progress*, and *done*.  *  `driver` Returns the Atlassian account ID of the version driver.  *  `approvers` Returns a list containing approvers for this version.  Optional for create and update. | [optional]
**id** | Option<**String**> | The ID of the version. | [optional][readonly]
**issues_status_for_fix_version** | Option<[**models::VersionIssuesStatus**](VersionIssuesStatus.md)> | If the expand option `issuesstatus` is used, returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property contains a count of issues with a status other than *to do*, *in progress*, and *done*. | [optional][readonly]
**move_unfixed_issues_to** | Option<**String**> | The URL of the self link to the version to which all unfixed issues are moved when a version is released. Not applicable when creating a version. Optional when updating a version. | [optional]
**name** | Option<**String**> | The unique name of the version. Required when creating a version. Optional when updating a version. The maximum length is 255 characters. | [optional]
**operations** | Option<[**Vec<models::SimpleLink>**](SimpleLink.md)> | If the expand option `operations` is used, returns the list of operations available for this version. | [optional][readonly]
**overdue** | Option<**bool**> | Indicates that the version is overdue. | [optional][readonly]
**project** | Option<**String**> | Deprecated. Use `projectId`. | [optional]
**project_id** | Option<**i64**> | The ID of the project to which this version is attached. Required when creating a version. Not applicable when updating a version. | [optional]
**release_date** | Option<[**String**](string.md)> | The release date of the version. Expressed in ISO 8601 format (yyyy-mm-dd). Optional when creating or updating a version. | [optional]
**released** | Option<**bool**> | Indicates that the version is released. If the version is released a request to release again is ignored. Not applicable when creating a version. Optional when updating a version. | [optional]
**param_self** | Option<**String**> | The URL of the version. | [optional][readonly]
**start_date** | Option<[**String**](string.md)> | The start date of the version. Expressed in ISO 8601 format (yyyy-mm-dd). Optional when creating or updating a version. | [optional]
**user_release_date** | Option<**String**> | The date on which work on this version is expected to finish, expressed in the instance's *Day/Month/Year Format* date format. | [optional][readonly]
**user_start_date** | Option<**String**> | The date on which work on this version is expected to start, expressed in the instance's *Day/Month/Year Format* date format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


