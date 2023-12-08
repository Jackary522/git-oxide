//! `repo_objects`
//!
//! Provides the core structures and functions for handling Git objects in `rusty_git`.
//! This module includes definitions for Git objects like Blob, Tree, and Commit,
//! along with utilities to compress and decompress these objects, read them from the
//! file system, and write them back. The module forms the backbone of the Git-like
//! system by enabling the creation and manipulation of the fundamental Git object types.

use flate2::{bufread::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    fmt::Display,
    fs,
    io::{BufReader, Read, Write},
};

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    Tree,
    Blob,
    Commit,
}

/// Implements the Display trait for the `ObjectType` enum
impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Commit => write!(f, "commit"),
            ObjectType::Tree => write!(f, "tree"),
        }
    }
}

impl ObjectType {
    /// Returns a mode string for a given `ObjectType`
    fn mode(&self) -> String {
        match &self {
            ObjectType::Blob => String::from("100644"),
            ObjectType::Commit => panic!("No mode string"),
            ObjectType::Tree => String::from("40000"),
        }
    }
}

/// General Git object structure
pub struct Object {
    pub content: String,
    pub object_type: ObjectType,
}

/// Git compressed object structure
pub struct CompressedObject {
    pub content: Vec<u8>,
    pub object_type: ObjectType,
    pub hash: [u8; 20],
    pub hash_str: String,
    pub path: Option<String>,
}

// Reads a hashed Git object from `.git/objects` directory, decompresses it, and returns an `Object` instance containing the content and type of the Git object.
pub fn read_object(hash: &str) -> Object {
    let subpath = format!("{}/{}", &hash[..2], &hash[2..]);
    let path = format!("{}/{}", ".git/objects", subpath);

    let file = fs::File::open(path).expect("An error occurred while reading object.");
    let buffer = BufReader::new(file);

    let mut content = Vec::new();
    let mut decoder = ZlibDecoder::new(buffer);
    decoder
        .read_to_end(&mut content)
        .expect("An error occurred while decoding file.");

    let decompressed_content = String::from_utf8_lossy(&content);
    let (header, content) = decompressed_content
        .split_once('\0')
        .expect("Object header corrupted.");

    let (object_type, _) = header
        .split_once(' ')
        .expect("An error occured while parsing header.");
    let object_type = match object_type {
        "tree" => ObjectType::Tree,
        "blob" => ObjectType::Blob,
        "commit" => ObjectType::Commit,
        _ => panic!("Unsupported object type."),
    };

    Object {
        content: content.to_string(),
        object_type,
    }
}

/// Generates a SHA1 hash for the given content.
pub fn generate_hash(content: &Vec<u8>) -> [u8; 20] {
    Sha1::digest(content)[..20]
        .try_into()
        .expect("An error occurred while hashing content.")
}

/// Compresses the given content using Zlib compression.
pub fn compress_content(content: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(vec![], Compression::default());
    encoder
        .write_all(content)
        .expect("An error occurred during encoding.");
    encoder
        .finish()
        .expect("An error occurred during encoding.")
}

// Compresses a Git object, generating a header, calculating the hash, and returning a CompressedObject.
pub fn compress_object(
    content: &[u8],
    object_type: ObjectType,
    path: Option<&str>,
) -> CompressedObject {
    let header = object_header(content.len(), &object_type);
    let file_content = [header.as_bytes(), content].concat();
    let hash = generate_hash(&file_content);
    let compressed_content = compress_content(&file_content);

    CompressedObject {
        content: compressed_content,
        object_type,
        hash,
        hash_str: hex::encode(hash),
        path: path.map(String::from),
    }
}

/// Reads and returns the contents of the file at the given path.
pub fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).expect("An error occurred while reading a file.")
}

/// Reads the content of a file from the given path, compresses it into a Git object, and returns a `CompressedObject`.
pub fn read_and_compress(path: &str, object_type: ObjectType) -> CompressedObject {
    let content = read_file(path);
    compress_object(&content, object_type, Some(path))
}

// Generates a header string for a Git object based on its type and content length.
pub fn object_header(content_len: usize, object_type: &ObjectType) -> String {
    format!("{} {}\0", object_type, &content_len)
}

/// Writes the compressed content of a Git object to the .git/objects directory.
pub fn write_compressed(hash: &str, compressed_content: &Vec<u8>) {
    let (dir, object) = hash.split_at(2);
    let dir_path = format!("{}/{}", ".git/objects", dir);
    if fs::metadata(&dir_path).is_err() {
        fs::create_dir_all(&dir_path).expect("An error occured during directory creation.");
    }
    let object_path = format!("{}/{}", &dir_path, &object);

    fs::write(object_path, compressed_content).expect("An error occured while writing object.");
}

/// Formats a vector of `CompressedObject` instances into a Git tree object.
pub fn tree_format(tree: &Vec<CompressedObject>) -> Vec<u8> {
    let mut formatted_tree: Vec<Vec<u8>> = vec![];
    for object in tree {
        let path = match &object.path {
            Some(path) => path
                .split('/')
                .last()
                .expect("An error occurred while reading file name."),
            None => panic!("An error occurred while trying to locate file path."),
        };
        let mode = &object.object_type.mode();
        let entry = [
            mode.as_bytes(),
            vec![0x20].as_slice(),
            (path.as_bytes()),
            vec![0x00].as_slice(),
            object.hash.as_slice(),
        ]
        .concat();

        formatted_tree.push(entry);
    }
    formatted_tree.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_hash() {
        let content = b"test content";
        let hash = generate_hash(&content.to_vec());
        assert_eq!(hash.len(), 20);
    }

    #[test]
    fn test_compress_content() {
        let content = b"test content";
        let compressed = compress_content(content);
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_compress_object() {
        let content = b"test content";
        let compressed_object = compress_object(content, ObjectType::Blob, None);
        assert_eq!(compressed_object.object_type, ObjectType::Blob);
        assert!(!compressed_object.content.is_empty());
        assert_eq!(compressed_object.hash.len(), 20);
    }

    #[test]
    fn test_object_header() {
        let header = object_header(12, &ObjectType::Blob);
        assert_eq!(header, "blob 12\0");
    }

    #[test]
    fn test_write_compressed() {
        let compressed = b"compressed content";
        write_compressed("ab1234", &compressed.to_vec());
        // Verify file exists in `.git/objects/ab/1234`
    }
}
