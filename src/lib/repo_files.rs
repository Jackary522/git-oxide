//! `repo_files`
//!
//! Manages file operations for `rusty_git`. This module includes functions to interact
//! with the file system as part of the Git-like operations. Functions include printing
//! the content of Git objects based on their hash, creating tree objects from directories,
//! and initializing a new Git repository structure. This module works closely with
//! `repo_objects` to manage the file-based aspects of Git objects, providing a higher-level
//! interface for repository operations.

use super::repo_objects::{
    compress_object, read_and_compress, read_object, tree_format, write_compressed,
    CompressedObject, Object, ObjectType,
};
use std::fs;

/// Prints a hashed file to the console
pub fn cat_file(hash: &str) {
    let Object {
        content,
        object_type: _,
    } = read_object(hash);
    print!("{}", &content);
}

/// Hashes a given string as a Git object (Blob, Tree, Commit) and optionally writes it to the repository.
pub fn hash_object(object: &str, object_type: ObjectType, write: bool) {
    let CompressedObject {
        hash: _,
        content,
        object_type: _,
        hash_str,
        path: _,
    } = read_and_compress(object, object_type);

    if write {
        write_compressed(&hash_str, &content);
    } else {
        print!("{hash_str}");
    }
}

/// Lists the contents of a Git tree object. If `name_only` is true, it prints only the names of the items in the tree; otherwise, it prints detailed information.
pub fn ls_tree(name_only: bool, hash: &str) {
    let Object {
        content,
        object_type,
    } = read_object(hash);

    assert!(
        !(object_type != ObjectType::Tree),
        "The provided object is not a tree object."
    );

    let output = if name_only {
        let mut items = vec![];
        for item in content.split(' ') {
            match item.split_once('\0') {
                Some((name, _)) => items.push(name),
                None => continue,
            };
        }
        items.join("\n")
    } else {
        content
    };

    println!("{}", &output);
}

/// Creates a compressed Git tree object from a directory path. It reads the directory, compresses its contents as Git objects, and then forms a tree object.
pub fn create_tree(path: &str) -> CompressedObject {
    let entries = fs::read_dir(path).expect("An error occurred while reading a directory.");
    let mut objects: Vec<CompressedObject> = vec![];

    for entry in entries {
        let entry = entry.expect("An error occurred while reading an entry.");
        let metadata = entry
            .metadata()
            .expect("An error occurred while fetching entry metadata.");
        let entry_path = entry.path();
        let entry_path = entry_path
            .to_str()
            .expect("An error occured during path transformation.");
        if entry_path == path || entry_path.contains(".git") {
            continue;
        }

        if metadata.is_file() {
            let object = read_and_compress(entry_path, ObjectType::Blob);
            write_compressed(&object.hash_str, &object.content);
            objects.push(object);
        } else if metadata.is_dir() {
            let object = create_tree(entry_path);
            objects.push(object);
        }
    }

    objects.sort_by(|a, b| a.path.cmp(&b.path));

    let tree_content = tree_format(&objects);
    let compressed_tree = compress_object(&tree_content, ObjectType::Tree, Some(path));
    write_compressed(&compressed_tree.hash_str, &compressed_tree.content);

    compressed_tree
}

/// Prints the hash string of a compressed Git tree object for the current directory.
pub fn print_tree() {
    let compressed_tree = create_tree("./");
    println!("{}", compressed_tree.hash_str);
}
