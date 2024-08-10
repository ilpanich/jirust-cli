# JiraExpressionComplexity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expensive_operations** | **String** | Information that can be used to determine how many [expensive operations](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/#expensive-operations) the evaluation of the expression will perform. This information may be a formula or number. For example:   *  `issues.map(i => i.comments)` performs as many expensive operations as there are issues on the issues list. So this parameter returns `N`, where `N` is the size of issue list.  *  `new Issue(10010).comments` gets comments for one issue, so its complexity is `2` (`1` to retrieve issue 10010 from the database plus `1` to get its comments). | 
**variables** | Option<**std::collections::HashMap<String, String>**> | Variables used in the formula, mapped to the parts of the expression they refer to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


