
use configparser::ini::Ini;
use std::io::Write;
use std::{fs, process};
use std::fs::File;
use std::path::{Path, PathBuf};

use super::UserInput;

// TODO Remove .unwraps with Ok() => ... or println!(ErrMsg); process:exit(1);

pub struct GitRepo {
    worktree: PathBuf,
    gitdir: PathBuf,
    config: Ini,
}

impl GitRepo {
    fn new(path: PathBuf, force: bool) -> Self {
        let temp_gitdir_string: String = format!("{}/.git", &path.to_str().unwrap().to_string());

        GitRepo {
            worktree: path.clone(),
            gitdir: PathBuf::from(temp_gitdir_string),
            config: Ini::new(),
        }
    }
}

/// Computes the path under the repo's gitdir.
fn repo_path(repo: &GitRepo, path: Vec<String>) -> PathBuf {
    let git_dir: PathBuf = repo.gitdir.clone().join(path.join("/"));
    git_dir
}

pub fn repo_file(repo: &GitRepo, path: Vec<String>, mkdir: bool) -> PathBuf {
    if let Some(p) = repo_dir(repo, (&path[0..path.len() - 1]).to_vec(), mkdir) {
        return repo_path(repo, path);
    }
    PathBuf::new()
}

/// Same as repo_path, but mkdir path if absent if mkdir
fn repo_dir(repo: &GitRepo, path: Vec<String>, mkdir: bool) -> Option<PathBuf> {
    let path: PathBuf = repo_path(&repo, path);

    if path.exists() {
        if path.is_dir() {
            return Some(path);
        } else {
            println!("Not a directory {:?}", path);
            process::exit(1);
        }
    }

    if mkdir {
        fs::create_dir_all(&path).unwrap();
        return Some(path);
    }

    None
}

/// Create a new repo at path
pub fn repo_create(path: &str) -> GitRepo {
    let repo: GitRepo = GitRepo::new(PathBuf::from(path), true);

    if repo.worktree.exists() {
        if !repo.worktree.is_dir() {
            println!("{path} is not a directory!");
            process::exit(1);
        }
        if repo.gitdir.exists() && repo.gitdir.read_dir().unwrap().next().is_some() {
            println!("{path} is not empty!");
            process::exit(1);
        }
    } else {
        fs::create_dir_all(&repo.worktree).unwrap();
    }

    assert!(repo_dir(&repo, vec!["branches".to_string()], true).is_some());
    assert!(repo_dir(&repo, vec!["objects".to_string()], true).is_some());
    assert!(repo_dir(&repo, vec!["refs".to_string(), "tags".to_string()], true).is_some());
    assert!(repo_dir(&repo, vec!["refs".to_string(), "heads".to_string()], true).is_some());

    // * .git/description
    {
        let path: PathBuf = repo_file(&repo, vec!["description".to_string()], true);
        let mut file: File = File::create(path).unwrap();
        file.write_all(
            b"Unnamed repository; edit this file 'description' to name the repository.\n",
        )
        .unwrap();
    }

    // * .git/HEAD
    {
        let path: PathBuf = repo_file(&repo, vec!["HEAD".to_string()], true);
        let mut file: File = File::create(path).unwrap();
        file.write_all(b"ref: refs/heads/main\n").unwrap();
    }

    // * .git/config
    {
        let path: PathBuf = repo_file(&repo, vec!["config".to_string()], true);
        let config: Ini = repo_default_config();
        config.write(path).unwrap();
    }

    repo
}

fn repo_default_config() -> Ini {
    let mut ret: Ini = Ini::new();

    ret.set("core", "repositoryformatversion", Some("0.0".to_string()));
    ret.set("core", "filemode", Some("false".to_string()));
    ret.set("core", "bare", Some("false".to_string()));

    ret
}

fn repo_find(path: &Path, required: bool) -> Option<GitRepo> {
    let real_path: PathBuf = fs::canonicalize(path).expect("Failed to get canonical path");

    if real_path.join(".git").is_dir() {
        return Some(GitRepo::new(real_path, false));
    }

    let parent: &std::path::Path = real_path.parent().unwrap_or(&real_path);

    if parent == real_path {
        if required {
            println!("No git directory.");
            process::exit(1);
        } else {
            return None;
        }
    }

    repo_find(&parent, required)
}
