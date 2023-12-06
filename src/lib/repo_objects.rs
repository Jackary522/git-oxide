use flate2::{bufread::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    fmt::Display,
    fs,
    io::{BufReader, Read, Write},
};

// TODO comment this enum
#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    Tree,
    Blob,
    Commit,
}

/// Implements the Display trait for the ObjectType enum
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
    /// Returns a mode string for a given ObjectType
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// ```
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

/// Reads a hashed Git object and returns the decoded object
///
/// # Example
///
/// ```rust
///
/// ```
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

/// Generates a content hash
///
/// # Example
///
/// ```rust
///
/// ```
pub fn generate_hash(content: &Vec<u8>) -> [u8; 20] {
    Sha1::digest(content)[..20]
        .try_into()
        .expect("An error occurred while hashing content.")
}

/// Compresses a Git object's contents
///
/// # Example
///
/// ```rust
///
/// ```
pub fn compress_content(content: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(vec![], Compression::default());
    encoder
        .write_all(content)
        .expect("An error occurred during encoding.");
    encoder
        .finish()
        .expect("An error occurred during encoding.")
}

/// Compresses a Git object
///
/// # Example
///
/// ```rust
///
/// ```
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

/// Utility for reading a file
///
/// # Example
///
/// ```rust
///
/// ```
pub fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).expect("An error occurred while reading a file.")
}

/// Reads an object and returns it compressed
///
/// # Example
///
/// ```rust
///
/// ```
pub fn read_and_compress(path: &str, object_type: ObjectType) -> CompressedObject {
    let content = read_file(path);
    compress_object(&content, object_type, Some(path))
}

/// Generates an object header
///
/// # Example
///
/// ```rust
///
/// ```
pub fn object_header(content_len: usize, object_type: &ObjectType) -> String {
    format!("{} {}\0", object_type, &content_len)
}

/// Writes compressed content to a file
///
/// # Example
///
/// ```rust
///
/// ```
pub fn write_compressed(hash: &str, compressed_content: &Vec<u8>) {
    let (dir, object) = hash.split_at(2);
    let dir_path = format!("{}/{}", ".git/objects", dir);
    if fs::metadata(&dir_path).is_err() {
        fs::create_dir_all(&dir_path).expect("An error occured during directory creation.");
    }
    let object_path = format!("{}/{}", &dir_path, &object);

    fs::write(object_path, compressed_content).expect("An error occured while writing object.");
}

/// Formats a Git tree object
///
/// # Example
///
/// ```rust
///
/// ```
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
