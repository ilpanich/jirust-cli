#[macro_export]
macro_rules! jira_doc_std_field {
    ($v:expr) => {
        format!(
            "{{\"body\":{{\"content\":[{{\"content\":[{{\"text\":\"{}\",\"type\":\"text\"}}],\"type\":\"paragraph\"}}],\"type\":\"doc\",\"version\":1}}}}", $v).as_str()
    }
}
