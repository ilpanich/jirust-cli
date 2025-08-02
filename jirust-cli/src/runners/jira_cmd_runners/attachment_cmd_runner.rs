use jira_v3_openapi::apis::configuration::Configuration;
use jira_v3_openapi::apis::issue_attachments_api;
use serde_json::Value;

use crate::config::config_file::{AuthData, ConfigFile};

/// Attachment command runner
/// This struct is responsible for running the issue attachment command
/// It uses the Jira API to perform the operations
///
/// # Fields
///
/// * `cfg` - Configuration object
pub struct AttachmentCmdRunner {
    /// Configuration object
    cfg: Configuration,
}

/// Implementation of AttachmentCmdRunner
///
/// # Methods
///
/// * `new` - Creates a new instance of AttachmentCmdRunner
/// * `add_attachment` - Adds an attachment to a Jira issue
impl AttachmentCmdRunner {

    /// Creates a new instance of AttachmentCmdRunner
    ///
    /// # Arguments
    ///
    /// * `cfg_file` - Configuration file
    ///
    /// # Returns
    ///
    /// * `AttachmentCmdRunner` - Instance of AttachmentCmdRunner
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::attachment_cmd_runner::AttachmentCmdRunner;
    /// use toml::Table;
    ///
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    ///
    /// let attachment_cmd_runner = AttachmentCmdRunner::new(&cfg_file);
    /// ```
    pub fn new(cfg_file: &ConfigFile) -> AttachmentCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        AttachmentCmdRunner { cfg: config }
    }

    /// Adds an attachment to a Jira issue
    ///
    /// # Arguments
    ///
    /// * `params` - AttachmentCmdParams struct
    ///
    /// # Returns
    ///
    /// * `Result<Value, Box<dyn std::error::Error>>` - Result of the operation
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::attachment_cmd_runner::{AttachmentCmdRunner, AttachmentCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let attachment_cmd_runner = AttachmentCmdRunner::new(&cfg_file);
    /// let mut params = AttachmentCmdParams::new();
    /// params.issue_id_or_key = "ISSUE-1".to_string();
    /// params.file_content = vec![1, 2, 3];
    /// params.file_name = "test.txt".to_string();
    ///
    /// let result = attachment_cmd_runner.add_attachment(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn add_attachment(
        &self,
        params: AttachmentCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let response = issue_attachments_api::add_attachment(
            &self.cfg,
            &params.issue_id_or_key,
            params.file_content,
            &params.file_name,
        )
        .await?;

        Ok(serde_json::to_value(response)?)
    }
}

/// Attachment command parameters
///
/// # Fields
///
/// * `issue_id_or_key` - Jira issue ID or key
/// * `file_content` - Content of the file to attach
/// * `file_name` - Name of the file to attach
pub struct AttachmentCmdParams {
    /// Jira issue ID or key
    pub issue_id_or_key: String,
    /// Content of the file to attach
    pub file_content: Vec<u8>,
    /// Name of the file to attach
    pub file_name: String,
}

/// Implementation of AttachmentCmdParams struct
///
/// # Methods
///
/// * `new` - Creates a new AttachmentCmdParams instance
impl AttachmentCmdParams {
    /// Creates a new AttachmentCmdParams instance
    ///
    /// # Returns
    ///
    /// * `AttachmentCmdParams` - Attachment command parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::attachment_cmd_runner::AttachmentCmdParams;
    ///
    /// let params = AttachmentCmdParams::new();
    /// ```
    pub fn new() -> AttachmentCmdParams {
        AttachmentCmdParams {
            issue_id_or_key: "".to_string(),
            file_content: Vec::new(),
            file_name: "".to_string(),
        }
    }
}

/// Default implementation for AttachmentCmdParams struct
impl Default for AttachmentCmdParams {
    /// Creates a default AttachmentCmdParams instance
    ///
    /// # Returns
    ///
    /// A AttachmentCmdParams instance with default values
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::attachment_cmd_runner::AttachmentCmdParams;
    ///
    /// let params = AttachmentCmdParams::default();
    ///
    /// assert_eq!(params.issue_id_or_key, "".to_string());
    /// assert_eq!(params.file_content, Vec::<u8>::new());
    /// assert_eq!(params.file_name, "".to_string());
    /// ```
    fn default() -> Self {
        AttachmentCmdParams::new()
    }
}

// Placeholder for AttachmentArgs - you'll need to define this in your args module
use crate::args::commands::AttachmentArgs;

/// Implementation of From trait for AttachmentCmdParams struct
/// to convert AttachmentArgs struct to AttachmentCmdParams struct
impl From<&AttachmentArgs> for AttachmentCmdParams {
    /// Converts AttachmentArgs struct to AttachmentCmdParams struct
    /// to create a new AttachmentCmdParams instance
    ///
    /// # Arguments
    ///
    /// * `value` - AttachmentArgs struct
    ///
    /// # Returns
    ///
    /// * `AttachmentCmdParams` - Attachment command parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::attachment_cmd_runner::AttachmentCmdParams;
    /// use jirust_cli::args::commands::AttachmentArgs;
    /// use std::fs;
    /// use std::io::Write;
    ///
    /// let file_path = "test_attachment.txt";
    /// let mut file = fs::File::create(file_path).unwrap();
    /// file.write_all(b"Hello, world!").unwrap();
    ///
    /// let attachment_args = AttachmentArgs {
    ///    attachment_act: jirust_cli::args::commands::AttachmentActionValues::Add,
    ///    issue_key: "issue_key".to_string(),
    ///    attachment_file: file_path.to_string(),
    ///    output: jirust_cli::args::commands::OutputArgs::default(),
    /// };
    ///
    /// let params = AttachmentCmdParams::from(&attachment_args);
    ///
    /// assert_eq!(params.issue_id_or_key, "issue_key".to_string());
    /// assert_eq!(params.file_name, "test_attachment.txt".to_string());
    ///
    /// fs::remove_file(file_path).unwrap();
    /// ```
    fn from(value: &AttachmentArgs) -> Self {
        use std::fs;
        let file_content = fs::read(&value.attachment_file).expect("Unable to read file");
        let file_name = value.attachment_file.split('/').next_back().unwrap_or("").to_string();

        AttachmentCmdParams {
            issue_id_or_key: value.issue_key.clone(),
            file_content,
            file_name,
        }
    }
}

