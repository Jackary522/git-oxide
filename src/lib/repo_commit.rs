//! `repo_commit`
//!
//! Handles high-level Git operations like committing and cloning in `rusty_git`.
//! This module includes functionality to create new commit objects with detailed metadata
//! and to clone existing Git repositories. The `commit_tree` function is pivotal for
//! generating new commit objects representing changes in the repository, while `clone_repo`
//! enables the replication of remote repositories locally. These functions abstract complex
//! Git operations into simpler, more manageable Rust functions.

use super::repo_objects::{compress_object, write_compressed};
use git2::Repository;
use std::time::{SystemTime, UNIX_EPOCH};

/// Creates a Git commit object using provided parameters such as parent commit hash, commit message, and tree hash. The commit is then written to a file in the Git repository.
pub fn commit_tree(parent: Option<&str>, message: &str, tree_hash: &str) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("An error occurred while computing current timestamp.")
        .as_millis();

    let author = format!("Jack Hatton <jack.hatton522@gmail.com> {timestamp} +0300");

    let parent = parent.unwrap_or(" ");

    let content = format!(
        "tree {tree_hash}\n\
        parent {parent}\n\
        author {author}\n\
        committer {author}\n\n\
        {message}\n"
    );

    let compressed_object = compress_object(
        content.as_bytes(),
        super::repo_objects::ObjectType::Commit,
        None,
    );

    println!("{}", &compressed_object.hash_str);

    write_compressed(&compressed_object.hash_str, &compressed_object.content);
}

/// Clones a Git repository from a given URL into the current directory.
pub fn clone_repo(url: &str) {
    Repository::clone(url, ".")
        .expect("An error has occurred while attempting the clone the repository.");
}
