#[cfg(test)]
mod tests {
    use crate::utils::changelog_extractor::ChangelogExtractor;
    use crate::jira_doc_std_field;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_changelog_extractor_creation() {
        let extractor = ChangelogExtractor::new("CHANGELOG.md".to_string());
        assert_eq!(extractor.changelog_file, "CHANGELOG.md");
    }

    #[test]
    fn test_changelog_extractor_with_different_paths() {
        let extractor1 = ChangelogExtractor::new("/path/to/CHANGELOG.md".to_string());
        let extractor2 = ChangelogExtractor::new("./docs/CHANGELOG.md".to_string());
        let extractor3 = ChangelogExtractor::new("CHANGELOG.txt".to_string());

        assert_eq!(extractor1.changelog_file, "/path/to/CHANGELOG.md");
        assert_eq!(extractor2.changelog_file, "./docs/CHANGELOG.md");
        assert_eq!(extractor3.changelog_file, "CHANGELOG.txt");
    }

    #[test]
    fn test_extract_version_changelog_with_valid_format() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(temp_file, "# Changelog").expect("Failed to write to temp file");
        writeln!(temp_file, "").expect("Failed to write to temp file");
        writeln!(temp_file, "## [1.0.1] 2023-12-01").expect("Failed to write to temp file");
        writeln!(temp_file, "### Added").expect("Failed to write to temp file");
        writeln!(temp_file, "- New feature 1").expect("Failed to write to temp file");
        writeln!(temp_file, "- New feature 2").expect("Failed to write to temp file");
        writeln!(temp_file, "").expect("Failed to write to temp file");
        writeln!(temp_file, "## [1.0.0] 2023-11-01").expect("Failed to write to temp file");
        writeln!(temp_file, "### Added").expect("Failed to write to temp file");
        writeln!(temp_file, "- Initial release").expect("Failed to write to temp file");
        temp_file.flush().expect("Failed to flush temp file");

        let extractor = ChangelogExtractor::new(temp_file.path().to_string_lossy().to_string());
        let result = extractor.extract_version_changelog();

        assert!(result.is_ok());
        let changelog_text = result.unwrap();
        assert!(changelog_text.contains("### Added"));
        assert!(changelog_text.contains("- New feature 1"));
        assert!(changelog_text.contains("- New feature 2"));
        assert!(!changelog_text.contains("- Initial release"));
    }

    #[test]
    fn test_extract_version_changelog_single_version() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(temp_file, "# Changelog").expect("Failed to write to temp file");
        writeln!(temp_file, "").expect("Failed to write to temp file");
        writeln!(temp_file, "## [1.0.0] 2023-11-01").expect("Failed to write to temp file");
        writeln!(temp_file, "### Added").expect("Failed to write to temp file");
        writeln!(temp_file, "- Initial release").expect("Failed to write to temp file");
        writeln!(temp_file, "- First version").expect("Failed to write to temp file");
        temp_file.flush().expect("Failed to flush temp file");

        let extractor = ChangelogExtractor::new(temp_file.path().to_string_lossy().to_string());
        let result = extractor.extract_version_changelog();

        assert!(result.is_ok());
        let changelog_text = result.unwrap();
        assert!(changelog_text.contains("### Added"));
        assert!(changelog_text.contains("- Initial release"));
        assert!(changelog_text.contains("- First version"));
    }

    #[test]
    fn test_extract_version_changelog_no_version_format() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(temp_file, "# Changelog").expect("Failed to write to temp file");
        writeln!(temp_file, "").expect("Failed to write to temp file");
        writeln!(temp_file, "## Version 1.0.0").expect("Failed to write to temp file");
        writeln!(temp_file, "- Some changes").expect("Failed to write to temp file");
        temp_file.flush().expect("Failed to flush temp file");

        let extractor = ChangelogExtractor::new(temp_file.path().to_string_lossy().to_string());
        let result = extractor.extract_version_changelog();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No version changelog available"));
    }

    #[test]
    fn test_extract_version_changelog_file_not_found() {
        let extractor = ChangelogExtractor::new("/nonexistent/file.md".to_string());
        let result = extractor.extract_version_changelog();

        assert!(result.is_err());
    }

    #[test]
    fn test_extract_issues_from_changelog() {
        let changelog_text = r#"
### Added
- Fixed bug JIR-123 with login
- Improved performance JIR-456
- Added feature JIR-789 for reporting

### Fixed
- Resolved JIR-101 issue with data
- Fixed JIR-102 and JIR-103 together
"#.to_string();

        let extractor = ChangelogExtractor::new("test.md".to_string());
        let project_key = "JIR".to_string();
        let result = extractor.extract_issues_from_changelog(&changelog_text, &project_key);

        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 6);
        assert!(issues.contains(&"JIR-123".to_string()));
        assert!(issues.contains(&"JIR-456".to_string()));
        assert!(issues.contains(&"JIR-789".to_string()));
        assert!(issues.contains(&"JIR-101".to_string()));
        assert!(issues.contains(&"JIR-102".to_string()));
        assert!(issues.contains(&"JIR-103".to_string()));
    }

    #[test]
    fn test_extract_issues_from_changelog_no_issues() {
        let changelog_text = r#"
### Added
- Fixed bug with login
- Improved performance
- Added feature for reporting
"#.to_string();

        let extractor = ChangelogExtractor::new("test.md".to_string());
        let project_key = "JIR".to_string();
        let result = extractor.extract_issues_from_changelog(&changelog_text, &project_key);

        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_extract_issues_from_changelog_different_project_key() {
        let changelog_text = r#"
### Added
- Fixed bug PROJ-100 with login
- Improved performance PROJ-200
- Added feature JIR-789 for reporting
"#.to_string();

        let extractor = ChangelogExtractor::new("test.md".to_string());
        let project_key = "PROJ".to_string();
        let result = extractor.extract_issues_from_changelog(&changelog_text, &project_key);

        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 2);
        assert!(issues.contains(&"PROJ-100".to_string()));
        assert!(issues.contains(&"PROJ-200".to_string()));
        assert!(!issues.contains(&"JIR-789".to_string()));
    }

    #[test]
    fn test_extract_issues_duplicate_issues() {
        let changelog_text = r#"
### Added
- Fixed bug JIR-123 with login
- Improved performance for JIR-123
- Referenced JIR-123 again
"#.to_string();

        let extractor = ChangelogExtractor::new("test.md".to_string());
        let project_key = "JIR".to_string();
        let result = extractor.extract_issues_from_changelog(&changelog_text, &project_key);

        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 3); // Should include duplicates
        assert!(issues.iter().filter(|&issue| issue == "JIR-123").count() == 3);
    }

    #[test]
    fn test_jira_doc_std_field_macro() {
        let result = jira_doc_std_field!("This is a test").to_string();
        let expected = r#"{"content":[{"content":[{"text":"This is a test","type":"text"}],"type":"paragraph"}],"type":"doc","version":1}"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jira_doc_std_field_macro_with_empty_string() {
        let result = jira_doc_std_field!("").to_string();
        let expected = r#"{"content":[{"content":[{"text":"","type":"text"}],"type":"paragraph"}],"type":"doc","version":1}"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jira_doc_std_field_macro_with_special_characters() {
        let result = jira_doc_std_field!("Test with symbols: @#$%^&*()").to_string();
        let expected = r#"{"content":[{"content":[{"text":"Test with symbols: @#$%^&*()","type":"text"}],"type":"paragraph"}],"type":"doc","version":1}"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jira_doc_std_field_macro_with_numbers() {
        let result = jira_doc_std_field!("Version 1.2.3 released").to_string();
        let expected = r#"{"content":[{"content":[{"text":"Version 1.2.3 released","type":"text"}],"type":"paragraph"}],"type":"doc","version":1}"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_version_changelog_with_special_characters() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(temp_file, "# Changelog").expect("Failed to write to temp file");
        writeln!(temp_file, "").expect("Failed to write to temp file");
        writeln!(temp_file, "## [1.0.1] 2023-12-01").expect("Failed to write to temp file");
        writeln!(temp_file, "### Added").expect("Failed to write to temp file");
        writeln!(temp_file, "- Feature with special chars: @#$%^&*()").expect("Failed to write to temp file");
        writeln!(temp_file, "- Unicode support: 测试文本 🚀").expect("Failed to write to temp file");
        writeln!(temp_file, "").expect("Failed to write to temp file");
        writeln!(temp_file, "## [1.0.0] 2023-11-01").expect("Failed to write to temp file");
        writeln!(temp_file, "### Added").expect("Failed to write to temp file");
        writeln!(temp_file, "- Initial release").expect("Failed to write to temp file");
        temp_file.flush().expect("Failed to flush temp file");

        let extractor = ChangelogExtractor::new(temp_file.path().to_string_lossy().to_string());
        let result = extractor.extract_version_changelog();

        assert!(result.is_ok());
        let changelog_text = result.unwrap();
        assert!(changelog_text.contains("@#$%^&*()"));
        assert!(changelog_text.contains("测试文本"));
        assert!(changelog_text.contains("🚀"));
    }

    #[test]
    fn test_changelog_extractor_multiple_instances() {
        let extractor1 = ChangelogExtractor::new("file1.md".to_string());
        let extractor2 = ChangelogExtractor::new("file2.md".to_string());
        let extractor3 = ChangelogExtractor::new("file3.md".to_string());

        assert_eq!(extractor1.changelog_file, "file1.md");
        assert_eq!(extractor2.changelog_file, "file2.md");
        assert_eq!(extractor3.changelog_file, "file3.md");
    }

    #[test]
    fn test_extract_issues_with_complex_patterns() {
        let changelog_text = r#"
### Changes
- Fixed JIR-1 simple case
- Resolved JIR-12 with two digits
- Updated JIR-123 with three digits
- Merged JIR-1234 with four digits
- Handled JIR-12345 with five digits
- Edge case (JIR-999) in parentheses
- Reference to JIR-888.
- Multiple JIR-777, JIR-666, and JIR-555 in same line
"#.to_string();

        let extractor = ChangelogExtractor::new("test.md".to_string());
        let project_key = "JIR".to_string();
        let result = extractor.extract_issues_from_changelog(&changelog_text, &project_key);

        assert!(result.is_ok());
        let issues = result.unwrap();
        
        // Should find all issue references
        assert!(issues.contains(&"JIR-1".to_string()));
        assert!(issues.contains(&"JIR-12".to_string()));
        assert!(issues.contains(&"JIR-123".to_string()));
        assert!(issues.contains(&"JIR-1234".to_string()));
        assert!(issues.contains(&"JIR-12345".to_string()));
        assert!(issues.contains(&"JIR-999".to_string()));
        assert!(issues.contains(&"JIR-888".to_string()));
        assert!(issues.contains(&"JIR-777".to_string()));
        assert!(issues.contains(&"JIR-666".to_string()));
        assert!(issues.contains(&"JIR-555".to_string()));
    }
}