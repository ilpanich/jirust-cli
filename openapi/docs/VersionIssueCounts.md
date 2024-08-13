# VersionIssueCounts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_field_usage** | Option<[**Vec<models::VersionUsageInCustomField>**](VersionUsageInCustomField.md)> | List of custom fields using the version. | [optional][readonly]
**issue_count_with_custom_fields_showing_version** | Option<**i64**> | Count of issues where a version custom field is set to the version. | [optional][readonly]
**issues_affected_count** | Option<**i64**> | Count of issues where the `affectedVersion` is set to the version. | [optional][readonly]
**issues_fixed_count** | Option<**i64**> | Count of issues where the `fixVersion` is set to the version. | [optional][readonly]
**param_self** | Option<**String**> | The URL of these count details. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


