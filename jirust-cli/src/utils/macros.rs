#[macro_export]
macro_rules! jira_doc_std_field {
    ($v:literal) => {
        serde_json::json!({"body":{"content":[{"content":[{"text":$v,"type":"text"}],"type":"paragraph"}],"type":"doc","version":1}})
    }
}
