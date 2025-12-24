#![cfg(feature = "attachment_scan")]

use anyhow::{Context, Result};
use git2::Repository;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use yara_x::{Compiler, Rules, Scanner};

use crate::config::config_file::YaraSection;

/// Source type for YARA rules
#[derive(Debug, Clone, PartialEq)]
enum SourceType {
    Git,
    Zip,
}

impl SourceType {
    /// Detect source type from URL
    fn detect(url: &str) -> Self {
        if url.ends_with(".git") {
            SourceType::Git
        } else if url.ends_with(".zip") {
            SourceType::Zip
        } else {
            // Default to Git for backward compatibility
            SourceType::Git
        }
    }
}

/// Internal configuration for YARA scanner paths
struct YaraConfig {
    rules_source: String,
    rules_dir: PathBuf,
    cache_file: PathBuf,
    cache_version_file: PathBuf,
    source_type: SourceType,
}

impl YaraConfig {
    /// Create from ConfigFile
    fn from_config_file(cfg: &YaraSection) -> Result<Self> {
        let base_dir = Self::get_base_dir()?;

        let rules_dir = base_dir.join(cfg.get_rules_directory());
        let cache_file = base_dir.join(cfg.get_cache_file());
        let cache_version_file = base_dir.join(cfg.get_cache_version_file());

        let rules_source = cfg.get_rules_source().to_string();
        let source_type = SourceType::detect(&rules_source);

        Ok(YaraConfig {
            rules_source,
            rules_dir,
            cache_file,
            cache_version_file,
            source_type,
        })
    }

    /// Create with defaults (for backward compatibility)
    fn default() -> Result<Self> {
        let base_dir = Self::get_base_dir()?;

        // Use existing constants as fallback
        let rules_source = "https://github.com/Yara-Rules/rules.git".to_string();
        let source_type = SourceType::Git;

        Ok(YaraConfig {
            rules_source,
            rules_dir: base_dir.join("yara-rules"),
            cache_file: base_dir.join("yara_rules.cache"),
            cache_version_file: base_dir.join("yara_rules.cache.version"),
            source_type,
        })
    }

    /// Get base directory (~/.jirust-cli/)
    fn get_base_dir() -> Result<PathBuf> {
        match env::var_os("HOME") {
            Some(home) => {
                let base = PathBuf::from(home).join(".jirust-cli");
                if !base.exists() {
                    fs::create_dir_all(&base).context("Failed to create .jirust-cli directory")?;
                }
                Ok(base)
            }
            None => anyhow::bail!("HOME environment variable not set"),
        }
    }
}

/// Structure to manage compiled YARA rules
pub struct CachedYaraScanner {
    rules: Rules,
    #[allow(dead_code)]
    config: YaraConfig,
}

impl CachedYaraScanner {
    /// Create scanner using configuration from ConfigFile
    pub async fn from_config(cfg: &YaraSection) -> Result<Self> {
        let config = YaraConfig::from_config_file(cfg)?;
        let rules = Self::load_or_compile_rules(&config).await?;
        Ok(Self { rules, config })
    }

    /// Generate the new scanner with defaults (backward compatibility):
    /// - If the YARA rules in the repo have not been updated -> load cache (~0.5s)
    /// - If the YARA rules in the repo have been updated -> rebuild & save in cache (~30s)
    pub async fn new() -> Result<Self> {
        let config = YaraConfig::default()?;
        let rules = Self::load_or_compile_rules(&config).await?;
        Ok(Self { rules, config })
    }

    /// Load cached rules or rebuilds them if required
    async fn load_or_compile_rules(config: &YaraConfig) -> Result<Rules> {
        let current_version = Self::get_current_version(config)?;
        let cached_version = Self::get_cached_version(config);

        // Check if the cache can be used
        if let (Some(cached), Some(current)) = (cached_version, &current_version) {
            if cached == *current && config.cache_file.exists() {
                println!("📦 Loading cached rules...");

                match Self::load_cached_rules(config) {
                    Ok(rules) => {
                        println!("✅ cached rules loaded (version: {})", &current[..8]);
                        return Ok(rules);
                    }
                    Err(e) => {
                        println!("⚠️  Cache corruption: {} - rebuilding...", e);
                    }
                }
            } else {
                println!("🔄 Rules updated, rebuilding...");
            }
        } else {
            println!("🔨 No cache found, building...");
        }

        let rules = Self::compile_all_rules(config).await?;

        if let Err(e) = Self::save_to_cache(config, &rules, &current_version) {
            eprintln!("⚠️  Can't save cache: {}", e);
        } else {
            println!("💾 Compiled rules cached!");
        }

        Ok(rules)
    }

    /// Get current version identifier for the rules
    fn get_current_version(config: &YaraConfig) -> Result<Option<String>> {
        match config.source_type {
            SourceType::Git => Self::get_git_version(config),
            SourceType::Zip => Self::get_zip_version(config),
        }
    }

    /// Get git commit hash as version
    fn get_git_version(config: &YaraConfig) -> Result<Option<String>> {
        if !config.rules_dir.exists() {
            return Ok(None);
        }

        let repo = Repository::open(&config.rules_dir).context("Can't open git repository")?;

        let head = repo.head().context("Can't read HEAD")?;

        let commit = head.peel_to_commit().context("Can't read commit")?;

        Ok(Some(commit.id().to_string()))
    }

    /// Get content hash as version for zip files
    fn get_zip_version(config: &YaraConfig) -> Result<Option<String>> {
        if !config.rules_dir.exists() {
            return Ok(None);
        }

        // Read version from metadata file if exists
        let version_marker = config.rules_dir.join(".version");
        if version_marker.exists() {
            return Ok(Some(fs::read_to_string(version_marker)?));
        }

        Ok(None)
    }

    /// Check the cached version
    fn get_cached_version(config: &YaraConfig) -> Option<String> {
        fs::read_to_string(&config.cache_version_file).ok()
    }

    /// Load cached rules
    fn load_cached_rules(config: &YaraConfig) -> Result<Rules> {
        let cache_bytes = fs::read(&config.cache_file).context("Can't read cache")?;

        let rules = Rules::deserialize(&cache_bytes).context("Can't read rules")?;

        Ok(rules)
    }

    /// Store compiled rules in cache
    fn save_to_cache(config: &YaraConfig, rules: &Rules, version: &Option<String>) -> Result<()> {
        let serialized = rules.serialize()?;

        fs::write(&config.cache_file, serialized).context("Can't write cache")?;

        if let Some(ver) = version {
            fs::write(&config.cache_version_file, ver).context("Can't store cache version")?;
        }

        Ok(())
    }

    /// Compile all YARA rules
    async fn compile_all_rules(config: &YaraConfig) -> Result<Rules> {
        if !config.rules_dir.exists() {
            println!(
                "Can't find YARA rules directory ({}).",
                config.rules_dir.display()
            );
            update_yara_rules_with_config(config)
                .await
                .context("Can't download YARA rules")?;
        }

        let mut compiler = Compiler::new();
        let mut compiled_count = 0;
        let mut skipped_count = 0;

        println!("🔨 Building YARA rules...");

        for entry in WalkDir::new(&config.rules_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() {
                let extension = path.extension().and_then(|s| s.to_str());
                if matches!(extension, Some("yar") | Some("yara")) {
                    match fs::read_to_string(path) {
                        Ok(content) => match compiler.add_source(&*content) {
                            Ok(_) => {
                                compiled_count += 1;
                                if compiled_count % 100 == 0 {
                                    print!(".");
                                    use std::io::Write;
                                    std::io::stdout().flush().ok();
                                }
                            }
                            Err(e) => {
                                skipped_count += 1;
                                if std::env::var("VERBOSE").is_ok() {
                                    eprintln!("\n⚠️  Skipped rule {}: {}", path.display(), e);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("\n⚠️  Can't read {}: {}", path.display(), e);
                            skipped_count += 1;
                        }
                    }
                }
            }
        }

        println!(
            "\n✅ Built {} rules (skipped: {})",
            compiled_count, skipped_count
        );

        if compiled_count == 0 {
            anyhow::bail!("No rules built");
        }

        let rules = compiler.build();
        Ok(rules)
    }

    /// Check a binary file using YARA rules
    pub fn scan_file<P: AsRef<Path>>(&self, file_path: P) -> Result<Vec<String>> {
        let path = file_path.as_ref();

        if !path.exists() {
            anyhow::bail!("File {} not found", path.display());
        }

        let file_content = fs::read(path).context(format!("Can't read file {}", path.display()))?;

        let mut scanner = Scanner::new(&self.rules);
        let scan_results = scanner.scan(&file_content).context("Error scanning file")?;

        let matches: Vec<String> = scan_results
            .matching_rules()
            .map(|rule| rule.identifier().to_string())
            .collect();

        Ok(matches)
    }

    /// Scan a memory buffer
    pub fn scan_buffer(&self, buffer: &[u8]) -> Result<Vec<String>> {
        let mut scanner = Scanner::new(&self.rules);
        let results = scanner.scan(buffer)?;

        let matches: Vec<String> = results
            .matching_rules()
            .map(|rule| rule.identifier().to_string())
            .collect();

        Ok(matches)
    }

    /// Force rules rebuilding (invalidate cache) - uses default config
    pub fn force_recompile() -> Result<()> {
        let config = YaraConfig::default()?;
        Self::force_recompile_internal(&config)
    }

    /// Force rules rebuilding (invalidate cache) with specific config (internal)
    fn force_recompile_internal(config: &YaraConfig) -> Result<()> {
        println!("🗑️  Deleting cache...");

        fs::remove_file(&config.cache_file).ok();
        fs::remove_file(&config.cache_version_file).ok();

        println!("✅ Cache deleted");
        Ok(())
    }
}

/// Update or download YARA rules based on source type
async fn update_yara_rules_with_config(config: &YaraConfig) -> Result<bool> {
    match config.source_type {
        SourceType::Git => update_git_rules(config),
        SourceType::Zip => update_zip_rules(config).await,
    }
}

/// Update git repository
fn update_git_rules(config: &YaraConfig) -> Result<bool> {
    if config.rules_dir.exists() {
        println!("📦 Git repository exists, checking for updates...");

        let repo = Repository::open(&config.rules_dir).context("Can't open local repository")?;

        let mut remote = repo
            .find_remote("origin")
            .context("Remote 'origin' not found")?;

        remote.fetch(&["main"], None, None).context("Fetch error")?;

        let fetch_head = repo.refname_to_id("FETCH_HEAD")?;
        let head = repo.head()?.target().unwrap();

        if fetch_head == head {
            println!("✅ Repository already up to date");
            return Ok(false);
        }

        let fetch_commit = repo.find_commit(fetch_head)?;
        repo.reset(fetch_commit.as_object(), git2::ResetType::Hard, None)
            .context("Update error")?;

        println!("✅ Repository updated");

        // Invalidate cache after update
        fs::remove_file(&config.cache_file).ok();
        fs::remove_file(&config.cache_version_file).ok();

        Ok(true)
    } else {
        println!("📥 Cloning git repository (this might take a while)...");

        Repository::clone(&config.rules_source, &config.rules_dir).context("Cloning error")?;

        println!(
            "✅ Repository cloned successfully to {}",
            config.rules_dir.display()
        );
        Ok(true)
    }
}

/// Download and extract zip rules
async fn update_zip_rules(config: &YaraConfig) -> Result<bool> {
    use sha2::{Digest, Sha256};
    use zip::ZipArchive;

    println!("📥 Downloading YARA rules from {}...", config.rules_source);

    // Download to memory
    let response = reqwest::get(&config.rules_source)
        .await
        .context(format!("Failed to download from {}", config.rules_source))?;

    if !response.status().is_success() {
        anyhow::bail!("Download failed with status: {}", response.status());
    }

    // Read response bytes
    let zip_bytes = response
        .bytes()
        .await
        .context("Failed to read response body")?;

    // Calculate hash for version tracking
    let mut hasher = Sha256::new();
    hasher.update(&zip_bytes);
    let new_version = format!("{:x}", hasher.finalize());

    // Check if we already have this version
    let version_marker = config.rules_dir.join(".version");
    if version_marker.exists() {
        let current_version = fs::read_to_string(&version_marker).ok();
        if current_version.as_deref() == Some(new_version.as_str()) {
            println!(
                "✅ Rules already up to date (version: {})",
                &new_version[..8]
            );
            return Ok(false);
        }
    }

    // Clean existing rules directory
    if config.rules_dir.exists() {
        fs::remove_dir_all(&config.rules_dir)
            .context("Failed to clean existing rules directory")?;
    }

    // Create rules directory
    fs::create_dir_all(&config.rules_dir).context("Failed to create rules directory")?;

    // Extract zip
    println!("📦 Extracting rules...");
    let cursor = std::io::Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(cursor).context("Failed to read zip archive")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).context("Failed to read zip entry")?;

        let outpath = match file.enclosed_name() {
            Some(path) => config.rules_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            // Directory
            fs::create_dir_all(&outpath).context("Failed to create directory")?;
        } else {
            // File
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).context("Failed to create parent directory")?;
            }

            let mut outfile = fs::File::create(&outpath).context("Failed to create file")?;
            std::io::copy(&mut file, &mut outfile).context("Failed to extract file")?;
        }
    }

    // Write version marker
    fs::write(&version_marker, &new_version).context("Failed to write version marker")?;

    println!(
        "✅ Rules extracted successfully (version: {})",
        &new_version[..8]
    );

    // Invalidate cache
    fs::remove_file(&config.cache_file).ok();
    fs::remove_file(&config.cache_version_file).ok();

    Ok(true)
}

/// Update YARA-Rules with default configuration (backward compatibility).
/// Returns Ok(true) if updated.
pub async fn update_yara_rules() -> Result<bool> {
    let config = YaraConfig::default()?;
    update_yara_rules_with_config(&config).await
}

/// Entrypoint
pub async fn scan_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>> {
    let scanner = CachedYaraScanner::new().await?;
    scanner.scan_file(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::config_file::YaraSection;
    use std::sync::Mutex;
    use tempfile::tempdir;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn detects_source_type() {
        assert_eq!(
            SourceType::detect("https://example.com/rules.git"),
            SourceType::Git
        );
        assert_eq!(
            SourceType::detect("https://example.com/rules.zip"),
            SourceType::Zip
        );
        assert_eq!(
            SourceType::detect("https://example.com/rules"),
            SourceType::Git
        );
    }

    #[tokio::test]
    async fn builds_rules_and_writes_cache_version() {
        let _guard = ENV_MUTEX.lock().unwrap();

        let temp_home = tempdir().expect("temp HOME");
        let base_dir = temp_home.path().join(".jirust-cli");
        let rules_dir = base_dir.join("rules");
        fs::create_dir_all(&rules_dir).expect("create rules dir");

        fs::write(rules_dir.join(".version"), "v1").expect("write version marker");
        fs::write(
            rules_dir.join("test_rule.yar"),
            r#"
rule CacheRule {
  strings:
    $a = "cache-hit"
  condition:
    $a
}
"#,
        )
        .expect("write yara rule");

        let previous_home = env::var("HOME").ok();
        env::set_var("HOME", temp_home.path());

        let section = YaraSection::new(
            "local_rules.zip".to_string(),
            "rules".to_string(),
            "yara_rules.cache".to_string(),
            "yara_rules.cache.version".to_string(),
        );

        let scanner = CachedYaraScanner::from_config(&section)
            .await
            .expect("scanner builds");
        let matches = scanner
            .scan_buffer(b"cache-hit")
            .expect("scan buffer succeeds");

        assert!(matches.contains(&"CacheRule".to_string()));
        assert!(base_dir.join("yara_rules.cache").exists());
        let version = fs::read_to_string(base_dir.join("yara_rules.cache.version"))
            .expect("version cache exists");
        assert_eq!(version, "v1");

        if let Some(prev) = previous_home {
            env::set_var("HOME", prev);
        } else {
            env::remove_var("HOME");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_cached_scanner() {
        let scanner = CachedYaraScanner::new().await.unwrap();

        // test file creation
        std::fs::write("/tmp/test_file.txt", b"Hello World").unwrap();

        let result = scanner.scan_file("/tmp/test_file.txt");
        assert!(result.is_ok());

        std::fs::remove_file("/tmp/test_file.txt").ok();
    }

    #[tokio::test]
    #[ignore]
    async fn test_cache_persistence() {
        // First scan - rules should be compiled and cached
        let scanner1 = CachedYaraScanner::new().await.unwrap();
        drop(scanner1);

        // Second scan - rules should be loaded from cache
        let start = std::time::Instant::now();
        let scanner2 = CachedYaraScanner::new().await.unwrap();
        let elapsed = start.elapsed();

        // Cache load should be fast (< 2 seconds)
        assert!(elapsed.as_secs() < 2);

        drop(scanner2);
    }
}
