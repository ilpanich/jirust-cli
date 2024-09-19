# BulkFetchIssueRequestBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expand** | Option<**Vec<String>**> | Use [expand](em>#expansion) to include additional information about issues in the response. Note that, unlike the majority of instances where `expand` is specified, `expand` is defined as a list of values. The expand options are:   *  `renderedFields` Returns field values rendered in HTML format.  *  `names` Returns the display name of each field.  *  `schema` Returns the schema describing a field type.  *  `changelog` Returns a list of recent updates to an issue, sorted by date, starting from the most recent. | [optional]
**fields** | Option<**Vec<String>**> | A list of fields to return for each issue, use it to retrieve a subset of fields. This parameter accepts a comma-separated list. Expand options include:   *  `*all` Returns all fields.  *  `*navigable` Returns navigable fields.  *  Any issue field, prefixed with a minus to exclude.  The default is `*navigable`.  Examples:   *  `summary,comment` Returns the summary and comments fields only.  *  `-description` Returns all navigable (default) fields except description.  *  `*all,-comment` Returns all fields except comments.  Multiple `fields` parameters can be included in a request.  Note: All navigable fields are returned by default. This differs from [GET issue](#api-rest-api-3-issue-issueIdOrKey-get) where the default is all fields. | [optional]
**fields_by_keys** | Option<**bool**> | Reference fields by their key (rather than ID). The default is `false`. | [optional]
**issue_ids_or_keys** | **Vec<String>** | An array of issue IDs or issue keys to fetch. You can mix issue IDs and keys in the same query. | 
**properties** | Option<**Vec<String>**> | A list of issue property keys of issue properties to be included in the results. A maximum of 5 issue property keys can be specified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


