use configparser::ini::Ini;
use std::fs;

/// Generates the `config.ini` file required for a Git repository
///
/// This function is only used by the `initialize_git_dir` function
fn default_ini() -> Ini {
    let mut config = Ini::new();
    config.set("core", "repositoryformatversion", Some("0".to_string()));
    config.set("core", "filemode", Some("true".to_string()));
    config.set("core", "bare", Some("false".to_string()));
    config.set("core", "ignorecase", Some("true".to_string()));
    config
}

/// Generates and writes the necessary Git directory stucture for Git
///
/// # Example:
///
/// ``` rust
///
/// ```
pub fn initialize_git_dir() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();

    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    let config = default_ini();
    config.write(".git/config").unwrap();

    println!("Initialized git directory");
}
