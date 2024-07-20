# \ProjectVersionsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_related_work**](ProjectVersionsApi.md#create_related_work) | **POST** /rest/api/3/version/{id}/relatedwork | Create related work
[**create_version**](ProjectVersionsApi.md#create_version) | **POST** /rest/api/3/version | Create version
[**delete_and_replace_version**](ProjectVersionsApi.md#delete_and_replace_version) | **POST** /rest/api/3/version/{id}/removeAndSwap | Delete and replace version
[**delete_related_work**](ProjectVersionsApi.md#delete_related_work) | **DELETE** /rest/api/3/version/{versionId}/relatedwork/{relatedWorkId} | Delete related work
[**delete_version**](ProjectVersionsApi.md#delete_version) | **DELETE** /rest/api/3/version/{id} | Delete version
[**get_project_versions**](ProjectVersionsApi.md#get_project_versions) | **GET** /rest/api/3/project/{projectIdOrKey}/versions | Get project versions
[**get_project_versions_paginated**](ProjectVersionsApi.md#get_project_versions_paginated) | **GET** /rest/api/3/project/{projectIdOrKey}/version | Get project versions paginated
[**get_related_work**](ProjectVersionsApi.md#get_related_work) | **GET** /rest/api/3/version/{id}/relatedwork | Get related work
[**get_version**](ProjectVersionsApi.md#get_version) | **GET** /rest/api/3/version/{id} | Get version
[**get_version_related_issues**](ProjectVersionsApi.md#get_version_related_issues) | **GET** /rest/api/3/version/{id}/relatedIssueCounts | Get version's related issues count
[**get_version_unresolved_issues**](ProjectVersionsApi.md#get_version_unresolved_issues) | **GET** /rest/api/3/version/{id}/unresolvedIssueCount | Get version's unresolved issues count
[**merge_versions**](ProjectVersionsApi.md#merge_versions) | **PUT** /rest/api/3/version/{id}/mergeto/{moveIssuesTo} | Merge versions
[**move_version**](ProjectVersionsApi.md#move_version) | **POST** /rest/api/3/version/{id}/move | Move version
[**update_related_work**](ProjectVersionsApi.md#update_related_work) | **PUT** /rest/api/3/version/{id}/relatedwork | Update related work
[**update_version**](ProjectVersionsApi.md#update_version) | **PUT** /rest/api/3/version/{id} | Update version



## create_related_work

> models::VersionRelatedWork create_related_work(id, version_related_work)
Create related work

Creates a related work for the given version. You can only create a generic link type of related works via this API. relatedWorkId will be auto-generated UUID, that does not need to be provided.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Resolve issues:* and *Edit issues* [Managing project permissions](https://confluence.atlassian.com/adminjiraserver/managing-project-permissions-938847145.html) for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**version_related_work** | [**VersionRelatedWork**](VersionRelatedWork.md) |  | [required] |

### Return type

[**models::VersionRelatedWork**](VersionRelatedWork.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_version

> models::Version create_version(version)
Create version

Creates a project version.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project the version is added to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | [**Version**](Version.md) |  | [required] |

### Return type

[**models::Version**](Version.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_and_replace_version

> serde_json::Value delete_and_replace_version(id, delete_and_replace_version_bean)
Delete and replace version

Deletes a project version.  Alternative versions can be provided to update issues that use the deleted version in `fixVersion`, `affectedVersion`, or any version picker custom fields. If alternatives are not provided, occurrences of `fixVersion`, `affectedVersion`, and any version picker custom field, that contain the deleted version, are cleared. Any replacement version must be in the same project as the version being deleted and cannot be the version being deleted.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version. | [required] |
**delete_and_replace_version_bean** | [**DeleteAndReplaceVersionBean**](DeleteAndReplaceVersionBean.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_related_work

> delete_related_work(version_id, related_work_id)
Delete related work

Deletes the given related work for the given version.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Resolve issues:* and *Edit issues* [Managing project permissions](https://confluence.atlassian.com/adminjiraserver/managing-project-permissions-938847145.html) for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version_id** | **String** | The ID of the version that the target related work belongs to. | [required] |
**related_work_id** | **String** | The ID of the related work to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_version

> delete_version(id, move_fix_issues_to, move_affected_issues_to)
Delete version

Deletes a project version.  Deprecated, use [ Delete and replace version](#api-rest-api-3-version-id-removeAndSwap-post) that supports swapping version values in custom fields, in addition to the swapping for `fixVersion` and `affectedVersion` provided in this resource.  Alternative versions can be provided to update issues that use the deleted version in `fixVersion` or `affectedVersion`. If alternatives are not provided, occurrences of `fixVersion` and `affectedVersion` that contain the deleted version are cleared.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version. | [required] |
**move_fix_issues_to** | Option<**String**> | The ID of the version to update `fixVersion` to when the field contains the deleted version. The replacement version must be in the same project as the version being deleted and cannot be the version being deleted. |  |
**move_affected_issues_to** | Option<**String**> | The ID of the version to update `affectedVersion` to when the field contains the deleted version. The replacement version must be in the same project as the version being deleted and cannot be the version being deleted. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_versions

> Vec<models::Version> get_project_versions(project_id_or_key, expand)
Get project versions

Returns all versions in a project. The response is not paginated. Use [Get project versions paginated](#api-rest-api-3-project-projectIdOrKey-version-get) if you want to get the versions in a project with pagination.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information in the response. This parameter accepts `operations`, which returns actions that can be performed on the version. |  |

### Return type

[**Vec<models::Version>**](Version.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_versions_paginated

> models::PageBeanVersion get_project_versions_paginated(project_id_or_key, start_at, max_results, order_by, query, status, expand)
Get project versions paginated

Returns a [paginated](#pagination) list of all versions in a project. See the [Get project versions](#api-rest-api-3-project-projectIdOrKey-versions-get) resource if you want to get a full list of versions without pagination.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `description` Sorts by version description.  *  `name` Sorts by version name.  *  `releaseDate` Sorts by release date, starting with the oldest date. Versions with no release date are listed last.  *  `sequence` Sorts by the order of appearance in the user interface.  *  `startDate` Sorts by start date, starting with the oldest date. Versions with no start date are listed last. |  |
**query** | Option<**String**> | Filter the results using a literal string. Versions with matching `name` or `description` are returned (case insensitive). |  |
**status** | Option<**String**> | A list of status values used to filter the results by version status. This parameter accepts a comma-separated list. The status values are `released`, `unreleased`, and `archived`. |  |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:   *  `issuesstatus` Returns the number of issues in each status category for each version.  *  `operations` Returns actions that can be performed on the specified version.  *  `driver` Returns the Atlassian account ID of the version driver.  *  `approvers` Returns a list containing the approvers for this version. |  |

### Return type

[**models::PageBeanVersion**](PageBeanVersion.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_related_work

> Vec<models::VersionRelatedWork> get_related_work(id)
Get related work

Returns related work items for the given version id.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version. | [required] |

### Return type

[**Vec<models::VersionRelatedWork>**](VersionRelatedWork.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version

> models::Version get_version(id, expand)
Get version

Returns a project version.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version. | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about version in the response. This parameter accepts a comma-separated list. Expand options include:   *  `operations` Returns the list of operations available for this version.  *  `issuesstatus` Returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property represents the number of issues with a status other than *to do*, *in progress*, and *done*.  *  `driver` Returns the Atlassian account ID of the version driver.  *  `approvers` Returns a list containing the Atlassian account IDs of approvers for this version. |  |

### Return type

[**models::Version**](Version.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version_related_issues

> models::VersionIssueCounts get_version_related_issues(id)
Get version's related issues count

Returns the following counts for a version:   *  Number of issues where the `fixVersion` is set to the version.  *  Number of issues where the `affectedVersion` is set to the version.  *  Number of issues where a version custom field is set to the version.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* project permission for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version. | [required] |

### Return type

[**models::VersionIssueCounts**](VersionIssueCounts.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version_unresolved_issues

> models::VersionUnresolvedIssuesCount get_version_unresolved_issues(id)
Get version's unresolved issues count

Returns counts of the issues and unresolved issues for the project version.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* project permission for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version. | [required] |

### Return type

[**models::VersionUnresolvedIssuesCount**](VersionUnresolvedIssuesCount.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_versions

> serde_json::Value merge_versions(id, move_issues_to)
Merge versions

Merges two project versions. The merge is completed by deleting the version specified in `id` and replacing any occurrences of its ID in `fixVersion` with the version ID specified in `moveIssuesTo`.  Consider using [ Delete and replace version](#api-rest-api-3-version-id-removeAndSwap-post) instead. This resource supports swapping version values in `fixVersion`, `affectedVersion`, and custom fields.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version to delete. | [required] |
**move_issues_to** | **String** | The ID of the version to merge into. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_version

> models::Version move_version(id, version_move_bean)
Move version

Modifies the version's sequence within the project, which affects the display order of the versions in Jira.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* project permission for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version to be moved. | [required] |
**version_move_bean** | [**VersionMoveBean**](VersionMoveBean.md) |  | [required] |

### Return type

[**models::Version**](Version.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_related_work

> models::VersionRelatedWork update_related_work(id, version_related_work)
Update related work

Updates the given related work. You can only update generic link related works via Rest APIs. Any archived version related works can't be edited.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Resolve issues:* and *Edit issues* [Managing project permissions](https://confluence.atlassian.com/adminjiraserver/managing-project-permissions-938847145.html) for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version to update the related work on. For the related work id, pass it to the input JSON. | [required] |
**version_related_work** | [**VersionRelatedWork**](VersionRelatedWork.md) |  | [required] |

### Return type

[**models::VersionRelatedWork**](VersionRelatedWork.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_version

> models::Version update_version(id, version)
Update version

Updates a project version.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the version. | [required] |
**version** | [**Version**](Version.md) |  | [required] |

### Return type

[**models::Version**](Version.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

