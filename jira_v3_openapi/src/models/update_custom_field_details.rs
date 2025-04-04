/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateCustomFieldDetails : Details of a custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateCustomFieldDetails {
    /// The description of the custom field. The maximum length is 40000 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the custom field. It doesn't have to be unique. The maximum length is 255 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The searcher that defines the way the field is searched in Jira. It can be set to `null`, otherwise you must specify the valid searcher for the field type, as listed below (abbreviated values shown):   *  `cascadingselect`: `cascadingselectsearcher`  *  `datepicker`: `daterange`  *  `datetime`: `datetimerange`  *  `float`: `exactnumber` or `numberrange`  *  `grouppicker`: `grouppickersearcher`  *  `importid`: `exactnumber` or `numberrange`  *  `labels`: `labelsearcher`  *  `multicheckboxes`: `multiselectsearcher`  *  `multigrouppicker`: `multiselectsearcher`  *  `multiselect`: `multiselectsearcher`  *  `multiuserpicker`: `userpickergroupsearcher`  *  `multiversion`: `versionsearcher`  *  `project`: `projectsearcher`  *  `radiobuttons`: `multiselectsearcher`  *  `readonlyfield`: `textsearcher`  *  `select`: `multiselectsearcher`  *  `textarea`: `textsearcher`  *  `textfield`: `textsearcher`  *  `url`: `exacttextsearcher`  *  `userpicker`: `userpickergroupsearcher`  *  `version`: `versionsearcher`
    #[serde(rename = "searcherKey", skip_serializing_if = "Option::is_none")]
    pub searcher_key: Option<SearcherKey>,
}

impl UpdateCustomFieldDetails {
    /// Details of a custom field.
    pub fn new() -> UpdateCustomFieldDetails {
        UpdateCustomFieldDetails {
            description: None,
            name: None,
            searcher_key: None,
        }
    }
}
/// The searcher that defines the way the field is searched in Jira. It can be set to `null`, otherwise you must specify the valid searcher for the field type, as listed below (abbreviated values shown):   *  `cascadingselect`: `cascadingselectsearcher`  *  `datepicker`: `daterange`  *  `datetime`: `datetimerange`  *  `float`: `exactnumber` or `numberrange`  *  `grouppicker`: `grouppickersearcher`  *  `importid`: `exactnumber` or `numberrange`  *  `labels`: `labelsearcher`  *  `multicheckboxes`: `multiselectsearcher`  *  `multigrouppicker`: `multiselectsearcher`  *  `multiselect`: `multiselectsearcher`  *  `multiuserpicker`: `userpickergroupsearcher`  *  `multiversion`: `versionsearcher`  *  `project`: `projectsearcher`  *  `radiobuttons`: `multiselectsearcher`  *  `readonlyfield`: `textsearcher`  *  `select`: `multiselectsearcher`  *  `textarea`: `textsearcher`  *  `textfield`: `textsearcher`  *  `url`: `exacttextsearcher`  *  `userpicker`: `userpickergroupsearcher`  *  `version`: `versionsearcher`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SearcherKey {
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:cascadingselectsearcher")]
    Cascadingselectsearcher,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:daterange")]
    Daterange,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:datetimerange")]
    Datetimerange,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:exactnumber")]
    Exactnumber,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:exacttextsearcher")]
    Exacttextsearcher,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:grouppickersearcher")]
    Grouppickersearcher,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:labelsearcher")]
    Labelsearcher,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:multiselectsearcher")]
    Multiselectsearcher,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:numberrange")]
    Numberrange,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:projectsearcher")]
    Projectsearcher,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:textsearcher")]
    Textsearcher,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:userpickergroupsearcher")]
    Userpickergroupsearcher,
    #[serde(rename = "com.atlassian.jira.plugin.system.customfieldtypes:versionsearcher")]
    Versionsearcher,
}

impl Default for SearcherKey {
    fn default() -> SearcherKey {
        Self::Cascadingselectsearcher
    }
}

