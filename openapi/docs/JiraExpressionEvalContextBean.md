# JiraExpressionEvalContextBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**board** | Option<**i64**> | The ID of the board that is available under the `board` variable when evaluating the expression. | [optional]
**custom** | Option<[**Vec<models::CustomContextVariable>**](CustomContextVariable.md)> | Custom context variables and their types. These variable types are available for use in a custom context:   *  `user`: A [user](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#user) specified as an Atlassian account ID.  *  `issue`: An [issue](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#issue) specified by ID or key. All the fields of the issue object are available in the Jira expression.  *  `json`: A JSON object containing custom content.  *  `list`: A JSON list of `user`, `issue`, or `json` variable types. | [optional]
**customer_request** | Option<**i64**> | The ID of the customer request that is available under the `customerRequest` variable when evaluating the expression. This is the same as the ID of the underlying Jira issue, but the customer request context variable will have a different type. | [optional]
**issue** | Option<[**models::IdOrKeyBean**](IdOrKeyBean.md)> | The issue that is available under the `issue` variable when evaluating the expression. | [optional]
**issues** | Option<[**models::JexpIssues**](JexpIssues.md)> | The collection of issues that is available under the `issues` variable when evaluating the expression. | [optional]
**project** | Option<[**models::IdOrKeyBean**](IdOrKeyBean.md)> | The project that is available under the `project` variable when evaluating the expression. | [optional]
**service_desk** | Option<**i64**> | The ID of the service desk that is available under the `serviceDesk` variable when evaluating the expression. | [optional]
**sprint** | Option<**i64**> | The ID of the sprint that is available under the `sprint` variable when evaluating the expression. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


