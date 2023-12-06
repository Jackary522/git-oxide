//! `repo_files`
//!
//! Handles file operations for `rusty_git`

use super::repo_objects::{
    compress_object, read_and_compress, read_object, tree_format, write_compressed,
    CompressedObject, Object, ObjectType,
};
use std::fs;

/// Prints a hashed file to the console
///
/// # Example
///
/// Basic usage:
///
/// ```rust
/// # use crate::lib::repo_files::cat_hash;
///
/// let hash = "d719fc22c485f069dea93469f4ea92ccd42cfeb7"; // TODO This doctest doesn't work
/// cat_file(hash);
/// ```
pub fn cat_file(hash: &str) {
    let Object {
        content,
        object_type: _,
    } = read_object(hash);
    print!("{}", &content);
}

/// Takes a hase and writes it to a file
///
/// This function is only used within the `hash_object` function
// fn hash_to_file(hash: &str, content: &[u8]) {
//     let mut encoder = ZlibEncoder::new(vec![], Compression::default());
//     encoder
//         .write_all(content)
//         .expect("An error occurred during encoding.");
//     let compressed_content = encoder
//         .finish()
//         .expect("An error occurred during encoding.");
// }

/// Hashes a string as a Git object (Blob, Tree, Commit)
///
/// # Example
///
/// ```rust
/// # use crate::repo_files::hash_object;
///
/// hash_object("foo", "blob", false);
/// ```
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

/// Lists a Git tree object
/// - either the tree with contents, or just leaf names
///
/// # Example
///
/// ```rust
///
/// ```
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

/// Writes the directory tree as a hash
pub fn write_tree() {
    let compressed_tree = create_tree("./");
    println!("{}", compressed_tree.hash_str);
}
