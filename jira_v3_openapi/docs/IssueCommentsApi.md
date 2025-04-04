# \IssueCommentsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_comment**](IssueCommentsApi.md#add_comment) | **POST** /rest/api/3/issue/{issueIdOrKey}/comment | Add comment
[**delete_comment**](IssueCommentsApi.md#delete_comment) | **DELETE** /rest/api/3/issue/{issueIdOrKey}/comment/{id} | Delete comment
[**get_comment**](IssueCommentsApi.md#get_comment) | **GET** /rest/api/3/issue/{issueIdOrKey}/comment/{id} | Get comment
[**get_comments**](IssueCommentsApi.md#get_comments) | **GET** /rest/api/3/issue/{issueIdOrKey}/comment | Get comments
[**get_comments_by_ids**](IssueCommentsApi.md#get_comments_by_ids) | **POST** /rest/api/3/comment/list | Get comments by IDs
[**update_comment**](IssueCommentsApi.md#update_comment) | **PUT** /rest/api/3/issue/{issueIdOrKey}/comment/{id} | Update comment



## add_comment

> models::Comment add_comment(issue_id_or_key, comment, expand)
Add comment

Adds a comment to an issue.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* and *Add comments* [ project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue containing the comment is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key of the issue. | [required] |
**comment** | [**Comment**](Comment.md) |  | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML. |  |

### Return type

[**models::Comment**](Comment.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment

> delete_comment(issue_id_or_key, id)
Delete comment

Deletes a comment.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue containing the comment is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Delete all comments*[ project permission](https://confluence.atlassian.com/x/yodKLg) to delete any comment or *Delete own comments* to delete comment created by the user,  *  If the comment has visibility restrictions, the user belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key of the issue. | [required] |
**id** | **String** | The ID of the comment. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment

> models::Comment get_comment(issue_id_or_key, id, expand)
Get comment

Returns a comment.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the comment.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the comment has visibility restrictions, the user belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key of the issue. | [required] |
**id** | **String** | The ID of the comment. | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML. |  |

### Return type

[**models::Comment**](Comment.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments

> models::PageOfComments get_comments(issue_id_or_key, start_at, max_results, order_by, expand)
Get comments

Returns all comments for an issue.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Comments are included in the response where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the comment.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key of the issue. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]
**order_by** | Option<**String**> | [Order](#ordering) the results by a field. Accepts *created* to sort comments by their created date. |  |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML. |  |

### Return type

[**models::PageOfComments**](PageOfComments.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments_by_ids

> models::PageBeanComment get_comments_by_ids(issue_comment_list_request_bean, expand)
Get comments by IDs

Returns a [paginated](#pagination) list of comments specified by a list of comment IDs.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Comments are returned where the user:   *  has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the comment.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_comment_list_request_bean** | [**IssueCommentListRequestBean**](IssueCommentListRequestBean.md) | The list of comment IDs. | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about comments in the response. This parameter accepts a comma-separated list. Expand options include:   *  `renderedBody` Returns the comment body rendered in HTML.  *  `properties` Returns the comment's properties. |  |

### Return type

[**models::PageBeanComment**](PageBeanComment.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_comment

> models::Comment update_comment(issue_id_or_key, id, comment, notify_users, override_editable_flag, expand)
Update comment

Updates a comment.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue containing the comment is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Edit all comments*[ project permission](https://confluence.atlassian.com/x/yodKLg) to update any comment or *Edit own comments* to update comment created by the user.  *  If the comment has visibility restrictions, the user belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key of the issue. | [required] |
**id** | **String** | The ID of the comment. | [required] |
**comment** | [**Comment**](Comment.md) |  | [required] |
**notify_users** | Option<**bool**> | Whether users are notified when a comment is updated. |  |[default to true]
**override_editable_flag** | Option<**bool**> | Whether screen security is overridden to enable uneditable fields to be edited. Available to Connect app users with the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) and Forge apps acting on behalf of users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). |  |[default to false]
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML. |  |

### Return type

[**models::Comment**](Comment.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

