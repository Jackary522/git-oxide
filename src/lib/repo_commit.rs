use super::repo_objects::{compress_object, write_compressed};
use git2::Repository;
use std::time::{SystemTime, UNIX_EPOCH};

/// Creates a Git commit object and writes it to a file
///
/// # Example
///
/// ```rust
///
/// ```
pub fn commit_tree(parent: &str, message: &str, tree_hash: &str) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("An error occurred while computing current timestamp.")
        .as_millis();

    let author = format!("Jack Hatton <jack.hatton522@gmail.com> {timestamp} +0300");

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

/// Clones an existing repository given the URL
///
/// # Example
///
/// ```rust
///
/// ```
pub fn clone_repo(url: &str) {
    Repository::clone(url, ".")
        .expect("An error has occurred while attempting the clone the repository.");
}
