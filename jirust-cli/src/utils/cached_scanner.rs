use anyhow::{Context, Result};
use git2::Repository;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use yara_x::{Compiler, Rules, Scanner};

const YARA_RULES_REPO: &str = "https://github.com/Yara-Rules/rules.git";
const RULES_DIR: &str = "./.jirust-cli/yara-rules";
const CACHE_FILE: &str = "./.jirust-cli/yara_rules.cache";
const CACHE_VERSION_FILE: &str = "./.jirust-cli/yara_rules.cache.version";

/// Structure to manage compiled YARA rules
pub struct CachedYaraScanner {
    rules: Rules,
}

impl CachedYaraScanner {
    /// Generate the new scanner:
    /// - If the YARA rules in the repo have not been updated -> load cache (~0.5s)
    /// - If the YARA rules in the repo have been updated -> rebuild & save in cache (~30s)
    pub fn new() -> Result<Self> {
        let rules = Self::load_or_compile_rules()?;
        Ok(Self { rules })
    }

    /// Load cached rules or rebuilds them if required
    fn load_or_compile_rules() -> Result<Rules> {
        let current_version = Self::get_repo_version()?;
        let cached_version = Self::get_cached_version();

        // Check if the cache can be used
        if let (Some(cached), Some(current)) = (cached_version, &current_version) {
            if cached == *current && Path::new(CACHE_FILE).exists() {
                println!("📦 Loading cached rules...");

                match Self::load_cached_rules() {
                    Ok(rules) => {
                        println!("✅ cached rules loaded (commit: {})", &current[..8]);
                        return Ok(rules);
                    }
                    Err(e) => {
                        println!("⚠️  Cache corruption: {} - rebuilding...", e);
                    }
                }
            } else {
                println!("🔄 Rule repository updated, rebuilding...");
            }
        } else {
            println!("🔨 No cache found, building...");
        }

        let rules = Self::compile_all_rules()?;

        if let Err(e) = Self::save_to_cache(&rules, &current_version) {
            eprintln!("⚠️  Can't save cache: {}", e);
        } else {
            println!("💾 Compiled rules cached!");
        }

        Ok(rules)
    }

    /// Check the current git commit hash in the repository
    fn get_repo_version() -> Result<Option<String>> {
        let rules_path = Path::new(RULES_DIR);

        if !rules_path.exists() {
            return Ok(None);
        }

        let repo = Repository::open(rules_path).context("Can't open repository")?;

        let head = repo.head().context("Can't read HEAD")?;

        let commit = head.peel_to_commit().context("Can't read commit")?;

        Ok(Some(commit.id().to_string()))
    }

    /// Check the cached version
    fn get_cached_version() -> Option<String> {
        fs::read_to_string(CACHE_VERSION_FILE).ok()
    }

    /// Load cached rules
    fn load_cached_rules() -> Result<Rules> {
        let cache_bytes = fs::read(CACHE_FILE).context("Can't read cache")?;

        let rules = Rules::deserialize(&cache_bytes).context("Can't read rules")?;

        Ok(rules)
    }

    /// Store compiled rules in cache
    fn save_to_cache(rules: &Rules, version: &Option<String>) -> Result<()> {
        let serialized = rules.serialize();

        fs::write(CACHE_FILE, serialized).context("Can't write cache")?;

        if let Some(ver) = version {
            fs::write(CACHE_VERSION_FILE, ver).context("Can't store cache version")?;
        }

        Ok(())
    }

    /// Compile all YARA rules
    fn compile_all_rules() -> Result<Rules> {
        let rules_path = Path::new(RULES_DIR);

        if !rules_path.exists() {
            anyhow::bail!("Can't find YARA rules directory ({}).", RULES_DIR);
        }

        let mut compiler = Compiler::new();
        let mut compiled_count = 0;
        let mut skipped_count = 0;

        println!("🔨 Building YARA rules...");

        for entry in WalkDir::new(rules_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() {
                let extension = path.extension().and_then(|s| s.to_str());
                if matches!(extension, Some("yar") | Some("yara")) {
                    match fs::read_to_string(path) {
                        Ok(content) => match compiler.add_source(&content) {
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
    pub fn scan_buffer(&self, buffer: &[u8], name: &str) -> Result<Vec<String>> {
        let mut scanner = Scanner::new(&self.rules);
        let results = scanner.scan(buffer)?;

        let matches: Vec<String> = results
            .matching_rules()
            .map(|rule| rule.identifier().to_string())
            .collect();

        Ok(matches)
    }

    /// Force rules rebuilding (invalidate cache)
    pub fn force_recompile() -> Result<()> {
        println!("🗑️  Deleting cache...");

        fs::remove_file(CACHE_FILE).ok();
        fs::remove_file(CACHE_VERSION_FILE).ok();

        println!("✅ Cache deleted");
        Ok(())
    }
}

/// Update YARA-Rules GitHub repository.
/// Returns Ok(true) if updated.
pub fn update_yara_rules() -> Result<bool> {
    let rules_path = Path::new(RULES_DIR);

    if rules_path.exists() {
        println!("📦 Repository YARA-Rules already downloaded, checking for updates...");

        let repo = Repository::open(rules_path).context("Can't open local repository")?;

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

        // Invalida la cache dopo l'aggiornamento
        fs::remove_file(CACHE_FILE).ok();
        fs::remove_file(CACHE_VERSION_FILE).ok();

        Ok(true)
    } else {
        println!("📥 Cloning YARA-Rules repository (it might take a while)...");

        Repository::clone(YARA_RULES_REPO, rules_path).context("Cloning error")?;

        println!("✅ Repository cloned succesfully in {}", RULES_DIR);
        Ok(true)
    }
}

/// Entrypoint
pub fn scan_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>> {
    let scanner = CachedYaraScanner::new()?;
    scanner.scan_file(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_cached_scanner() {
        let scanner = CachedYaraScanner::new().unwrap();

        // Crea un file di test
        std::fs::write("/tmp/test_file.txt", b"Hello World").unwrap();

        let result = scanner.scan_file("/tmp/test_file.txt");
        assert!(result.is_ok());

        std::fs::remove_file("/tmp/test_file.txt").ok();
    }

    #[test]
    #[ignore]
    fn test_cache_persistence() {
        // Prima scansione - compila
        let scanner1 = CachedYaraScanner::new().unwrap();
        drop(scanner1);

        // Seconda scansione - dovrebbe caricare dalla cache
        let start = std::time::Instant::now();
        let scanner2 = CachedYaraScanner::new().unwrap();
        let elapsed = start.elapsed();

        // Il caricamento dalla cache dovrebbe essere <1s
        assert!(elapsed.as_secs() < 2);

        drop(scanner2);
    }
}
