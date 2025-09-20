use std::process::Command;

use jirust_cli::config::config_file::ConfigFile;
use tempfile::tempdir;

#[test]
fn cli_config_show_uses_existing_config() {
    let binary = option_env!("CARGO_BIN_EXE_jirust-cli")
        .expect("binary path not available; ensure the jirust-cli bin is built");

    let tmp_home = tempdir().expect("create temp home");
    let config_dir = tmp_home.path().join(".jirust-cli");
    std::fs::create_dir_all(&config_dir).expect("create config dir");

    let config_path = config_dir.join("jirust-cli.toml");
    let mut config = ConfigFile::default();
    config.set_auth_key("dGVzdDphcGk=".to_string());
    config.set_jira_url("https://example.atlassian.net".to_string());
    config
        .write_to_file(config_path.to_str().expect("config path utf8"))
        .expect("write config");

    let output = Command::new(binary)
        .arg("config")
        .arg("show")
        .env("HOME", tmp_home.path())
        .output()
        .expect("run jirust-cli");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Auth token"));
    assert!(stdout.contains("Jira URL"));
}
