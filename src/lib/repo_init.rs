//! `repo_init`
//!
//! Handles the initialization process for new Git-like repositories in `rusty_git`.
//! This module is responsible for setting up the necessary directory structure and
//! configuration files required for a new repository. It includes functions to create
//! the .git directory, establish default configurations, and lay the foundational
//! structure for further Git operations.

use configparser::ini::Ini;
use std::fs;

/// Generates a default configuration for a Git repository. This includes standard settings like repository format version, file mode, bare repository flag, and case sensitivity.
fn default_ini() -> Ini {
    let mut config = Ini::new();
    config.set("core", "repositoryformatversion", Some("0".to_string()));
    config.set("core", "filemode", Some("true".to_string()));
    config.set("core", "bare", Some("false".to_string()));
    config.set("core", "ignorecase", Some("true".to_string()));
    config
}

/// Initializes the necessary directory structure for a Git repository in the current directory. It creates .git, .git/objects, and .git/refs directories, sets up the HEAD file, and writes a default configuration to .git/config.
pub fn initialize_git_dir() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();

    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    let config = default_ini();
    config.write(".git/config").unwrap();

    println!("Initialized git directory");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ini() {
        let config = default_ini();
        assert_eq!(config.get("core", "repositoryformatversion").unwrap(), "0");
        assert_eq!(config.get("core", "filemode").unwrap(), "true");
        assert_eq!(config.get("core", "bare").unwrap(), "false");
        assert_eq!(config.get("core", "ignorecase").unwrap(), "true");
    }
}
