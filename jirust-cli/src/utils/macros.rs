/// Macro to generate the Jira document field for a given text value
///
/// # Arguments
///
/// * `$v` - The text value to be used in the Jira document field
///
/// # Returns
///
/// * A string representing the Jira document field
///
/// # Example
///
/// ```
/// use jirust_cli::jira_doc_std_field;
///
/// assert_eq!(jira_doc_std_field!("This is a test"), "{\"body\":{\"content\":[{\"content\":[{\"text\":\"This is a test\",\"type\":\"text\"}],\"type\":\"paragraph\"}],\"type\":\"doc\",\"version\":1}}");
/// ```
#[macro_export]
macro_rules! jira_doc_std_field {
    ($v:expr) => {
        format!(
            "{{\"content\":[{{\"content\":[{{\"text\":\"{}\",\"type\":\"text\"}}],\"type\":\"paragraph\"}}],\"type\":\"doc\",\"version\":1}}", $v).as_str()
    }
}
