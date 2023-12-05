use flate2::bufread::ZlibDecoder;
use std::{
    fs,
    io::{BufReader, Read},
};

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    Tree,
    Blob,
    Commit,
}

pub struct Object {
    pub content: String,
    pub object_type: ObjectType,
}

pub fn read_object(hash: &str) -> Object {
    let subpath = format!("{}/{}", &hash[..2], &hash[2..]);
    let path = format!("{}/{}", ".git/objects", subpath);

    let file = fs::File::open(path).expect("An error occurred while reading object.");
    let buffer = BufReader::new(file);

    let mut decompressed_content = Vec::new();
    let mut decoder = ZlibDecoder::new(buffer);
    decoder
        .read_to_end(&mut decompressed_content)
        .expect("An error occurred while decoding file.");

    let decompressed_content = String::from_utf8_lossy(&decompressed_content);
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
