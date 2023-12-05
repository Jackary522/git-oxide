use crate::repo_obj::{read_object, Object, ObjectType};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fs, io::Write};

pub fn cat_file(hash: &str) {
    let Object {
        content,
        object_type: _,
    } = read_object(hash);
    print!("{}", &content);
}

fn hash_to_file(hash: &str, content: &[u8]) {
    let mut encoder = ZlibEncoder::new(vec![], Compression::default());
    encoder
        .write_all(content)
        .expect("An error occurred during encoding.");
    let compressed_content = encoder
        .finish()
        .expect("An error occurred during encoding.");

    let (dir, object) = &hash.split_at(2);
    let dir_path = format!("{}/{}", ".git/objects", dir);
    if fs::metadata(&dir_path).is_err() {
        fs::create_dir_all(&dir_path).expect("An error occured during directory creation.");
    }
    let object_path = format!("{}/{}", &dir_path, &object);

    fs::write(object_path, compressed_content).expect("An error occured while writing object.");
}

pub fn hash_object(object: &str, object_type: &str) {
    let file = fs::read(object);
    let file_contents = match file {
        Ok(file_contents) => file_contents,
        Err(err) => panic!("{}", err.to_string()),
    };

    let header = format!("{} {}\0", object_type, &file_contents.len());
    let contents_with_header = [header.as_bytes(), file_contents.as_slice()].concat();
    let hash = Sha1::digest(&contents_with_header)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();

    hash_to_file(&hash, &contents_with_header);
}

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
        for item in content.split('\0') {
            match item.split_once(' ') {
                Some((_, name)) => items.push(name),
                None => continue,
            };
        }
        items.join("\n")
    } else {
        content
    };

    println!("{}", &output);
}

pub fn write_tree() {}

pub fn commit_tree(tree_sha: &str, commit_sha: &str, message: &str) {
    println!("{tree_sha} {commit_sha} {message}");
}

pub fn clone_repo(url: &str) {
    println!("{url}");
}
