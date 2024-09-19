# SearchAndReconcileRequestBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expand** | Option<**String**> | Use [expand](em>#expansion) to include additional information about issues in the response. Note that, unlike the majority of instances where `expand` is specified, `expand` is defined as a list of values. The expand options are:   *  `renderedFields` Returns field values rendered in HTML format.  *  `names` Returns the display name of each field.  *  `schema` Returns the schema describing a field type.  *  `changelog` Returns a list of recent updates to an issue, sorted by date, starting from the most recent. | [optional]
**fields** | Option<**Vec<String>**> | A list of fields to return for each issue. Use it to retrieve a subset of fields. This parameter accepts a comma-separated list. Expand options include:   *  `*all` Returns all fields.  *  `*navigable` Returns navigable fields.  *  `id` Returns only issue IDs.  *  Any issue field, prefixed with a dash to exclude.  The default is `id`.  Examples:   *  `summary,comment` Returns the summary and comments fields only.  *  `*all,-comment` Returns all fields except comments.  Multiple `fields` parameters can be included in a request.  Note: By default, this resource returns IDs only. This differs from [GET issue](#api-rest-api-3-issue-issueIdOrKey-get) where the default is all fields. | [optional]
**fields_by_keys** | Option<**bool**> | Reference fields by their key (rather than ID). The default is `false`. | [optional]
**jql** | Option<**String**> | A [JQL](https://confluence.atlassian.com/x/egORLQ) expression. For performance reasons, this field requires a bounded query. A bounded query is a query with a search restriction.   *  Example of an unbounded query: `order by key desc`.  *  Example of a bounded query: `assignee = currentUser() order by key`. | [optional]
**max_results** | Option<**i32**> | The maximum number of items to return. Depending on search criteria, real number of items returned may be smaller. | [optional][default to 50]
**next_page_token** | Option<**String**> | The continuation token to fetch the next page. This token is provided by the response of this endpoint. | [optional]
**properties** | Option<**Vec<String>**> | A list of up to 5 issue properties to include in the results. This parameter accepts a comma-separated list. | [optional]
**reconcile_issues** | Option<**Vec<i64>**> | Strong consistency issue ids to be reconciled with search results. Accepts max 50 ids. All issues must exist. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


