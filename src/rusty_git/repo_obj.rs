use std::{path::PathBuf, io::{Read, Error, self, Write}, fs, str::from_utf8};
use flate2::{
    Compression,
    write::ZlibEncoder,
    read::ZlibDecoder,
};
use sha1::{Sha1, Digest};

use super::{GitRepo, repo_init::repo_file};

pub struct GitObject {
    data: Option<Vec<u8>>,
    fmt: Vec<u8>,
}

trait GitObjMethods {
    fn new(&mut self, data: Option<Vec<u8>>) {
        match data {
            None => self.init(),
            Some(data) => self.deserialize(data)
        };
    }
    fn deserialize(&mut self, data: Vec<u8>);
    fn serialize(&mut self, repo: Option<&GitRepo>) -> Vec<u8>;
    fn init(&mut self);
}

impl GitObjMethods for GitObject {
    fn deserialize(&mut self, data: Vec<u8>) {
        todo!()
    }

    fn serialize(&mut self, repo: Option<&GitRepo>) -> Vec<u8> {
        todo!()
    }

    fn init(&mut self) {
        todo!()
    }
}

impl GitObject {
    fn object_read(repo: &GitRepo, sha: &str) -> Result<Self, Error> {
        // TODO verify that the sha[0..2] & sha[2..] .to_string() appropriately handles as expected.
        let path: PathBuf = repo_file(&repo, vec!["objects".to_string(), sha[0..2].to_string(), sha[2..].to_string()], true);
    
        if !path.is_file() {
            return Err(Error::new(io::ErrorKind::NotFound, "File not found"));
        }
    
        let binding: Vec<u8> = fs::read(path)?;
        let mut decode: ZlibDecoder<&[u8]> = ZlibDecoder::new(&binding[..]);
        let mut raw: Vec<u8> = Vec::new();
        decode.read_to_end(&mut raw)?;

        let x = raw.iter()
            .position(|&b| b == b' ')
            .ok_or_else(|| Error::new(io::ErrorKind::InvalidData, "Invalid data format"))?;
        let fmt = raw[..x].to_vec();
        
        let y = raw.iter()
            .position(|&b| b == b'\x00')
            .ok_or_else(|| Error::new(io::ErrorKind::InvalidData, "Invalid data format"))?;
        let size = from_utf8(&raw[x+1..y]).unwrap()
            .parse::<usize>()
            .map_err(|_| Error::new(io::ErrorKind::InvalidData, "Invalid data size"))?;

        if size != raw.len() - y - 1 {
            return Err(Error::new(io::ErrorKind::InvalidData, format!("Malformed object {sha}: bad length")));
        } else {
            let mut obj = GitObject { data: Some(raw[y+1..].to_vec()), fmt };
            obj.new(None);
            Ok(obj)
        }
    }

    fn object_write(mut obj: GitObject, repo: Option<&GitRepo>) -> Result<String, Error> {
        let data = obj.serialize(repo);
        
        let header = format!("{} {}\x00", String::from_utf8(obj.fmt).unwrap(), data.len());
        let mut encoder: ZlibEncoder<Vec<u8>> = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(header.as_bytes())?;
        encoder.write_all(&data)?;
        let compressed = encoder.finish()?;

        let mut hasher = Sha1::new();
        hasher.update(&compressed);
        let hash = hasher.finalize();
        Ok(format!("{:X}", hash))
    }


}