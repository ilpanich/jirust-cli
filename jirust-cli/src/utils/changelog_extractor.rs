use regex::{Match, Regex};
use std::error::Error;
use std::fs;

/// Extracts the changelog text for a specific version to be used in the version description as release note
///
/// # Fields
///
/// * `changelog_file` - The path to the changelog file
pub struct ChangelogExtractor {
    pub changelog_file: String,
}

/// Implementation of the ChangelogExtractor
impl ChangelogExtractor {
    /// Creates a new ChangelogExtractor
    ///
    /// # Arguments
    ///
    /// * `changelog_file` - The path to the changelog file
    ///
    /// # Returns
    ///
    /// * A new ChangelogExtractor
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::utils::changelog_extractor::ChangelogExtractor;
    ///
    /// let changelog_extractor = ChangelogExtractor::new("CHANGELOG.md".to_string());
    ///
    /// assert_eq!(changelog_extractor.changelog_file, "CHANGELOG.md");
    /// ```
    pub fn new(changelog_file: String) -> Self {
        Self { changelog_file }
    }

    /// Extracts the changelog text for a specific version to be used in the version description as release note
    /// The version changelog text is extracted from the changelog file using the notes between the first and second version headers in markdown
    /// "Keep a Changelog" changelog formatted file
    ///
    /// # Returns
    ///
    /// * The version changelog text
    ///
    /// # Errors
    ///
    /// * If the changelog file cannot be read or the version changelog text cannot be extracted
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::utils::changelog_extractor::ChangelogExtractor;
    ///
    /// let changelog_extractor = ChangelogExtractor::new("CHANGELOG.md".to_string());
    ///
    /// let version_changelog_text = changelog_extractor.extract_version_changelog();
    /// ```
    pub fn extract_version_changelog(&self) -> Result<String, Box<dyn Error>> {
        let version_re = Regex::new(r"## \[\d+.\d+.\d+\] \d+\-\d+\-\d+\n").unwrap();
        let changelog = fs::read_to_string(&self.changelog_file)?;
        let matches: Vec<Match> = version_re.find_iter(&changelog).collect();
        if matches.is_empty() {
            return Err("No version changelog available".into());
        }
        let changelog_version_text_start = matches[0].range().end;
        let changelog_version_text_end = if matches.len() > 1 {
            matches[1].range().start
        } else {
            changelog.len()
        };
        let version_changelog_text = changelog
            [changelog_version_text_start..changelog_version_text_end]
            .replace("\\n", "\n")
            .replace("\\r", "\r");

        Ok(version_changelog_text.to_string())
    }

    /// Extracts the issues from the version changelog text
    /// The issues are extracted from the version changelog text using the project key and issue number
    ///
    /// # Arguments
    ///
    /// * `version_string` - The version changelog text
    /// * `project_key` - The project key
    ///
    /// # Returns
    ///
    /// * `Result<Vec<String>, Box<dyn Error>>` - The issues extracted from the version changelog text
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::utils::changelog_extractor::ChangelogExtractor;
    ///
    /// let changelog_extractor = ChangelogExtractor::new("CHANGELOG.md".to_string());
    ///
    /// let version_changelog_text = changelog_extractor.extract_version_changelog();
    /// let project_key = "JIR".to_string();
    /// let issues = changelog_extractor.extract_issues_from_changelog(&version_changelog_text.unwrap(), &project_key);
    /// ```
    pub fn extract_issues_from_changelog(
        &self,
        version_string: &String,
        project_key: &String,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let issue_re = Regex::new(format!(r"({}\-\d+)", *project_key).as_str()).unwrap();
        let mut issues: Vec<String> = vec![];
        for (_, [issue]) in issue_re
            .captures_iter((*version_string).as_str())
            .map(|issue| issue.extract())
        {
            issues.push(issue.to_string());
        }
        Ok(issues)
    }
}
