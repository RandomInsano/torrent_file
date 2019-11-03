extern crate serde;
extern crate serde_bencode;
extern crate serde_derive;

use std::path::Path;
use std::fs::File;

use serde_derive::{
    Serialize,
    Deserialize,
};
use serde_bencode::de;
use serde_bencode::ser;
use std::io::{
    Read,
    Write
};
use serde_bytes::ByteBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node(String, i64);

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePath {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    name: String,
    pieces: ByteBuf,
    #[serde(rename="piece length")]
    piece_length: i64,
    md5sum: Option<String>,
    length: Option<i64>,
    files: Option<Vec<FilePath>>,
    private: Option<u8>,
    path: Option<Vec<String>>,
    #[serde(rename="root hash")]
    root_hash: Option<String>,    
}

impl std::fmt::Debug for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: It'd be nice to know how Rust's "{:#?}" pretty formatter would
        //       work with the mess I've made below.
        write!(f, "Info {{")?;
        write!(f, "name: {:?}, ", self.name)?;
        write!(f, "piece_length: {:?}, ", self.piece_length)?;
        write!(f, "md5sum: {:?}, ", self.md5sum)?;
        write!(f, "length: {:?}, ", self.length)?;
        write!(f, "files: {:?}, ", self.files)?;
        write!(f, "private: {:?}, ", self.private)?;
        write!(f, "path: {:?}, ", self.path)?;
        write!(f, "root_hash: {:?}, ", self.root_hash)?;

        write!(f, "}}")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Torrent {
    info: Info,
    announce: Option<String>,
    nodes: Option<Vec<Node>>,
    encoding: Option<String>,
    httpseeds: Option<Vec<String>>,
    #[serde(rename="announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename="creation date")]
    creation_date: Option<i64>,
    #[serde(rename="comment")]
    comment: Option<String>,
    #[serde(rename="created by")]
    created_by: Option<String>,
    #[serde(rename="url-list")]
    pub url_list: Option<Vec<String>>,
}

impl Torrent {
    pub fn read(filename: &Path) -> Result<Torrent, serde_bencode::Error> {
        let mut buffer = Vec::new();
        let mut handle = File::open(filename).expect("file not found");

        handle.read_to_end(&mut buffer).unwrap();

        let torrent = de::from_bytes::<Torrent>(&buffer)?;

        Ok(torrent)
    }

    pub fn write(torrent: &Torrent, filename: &Path) -> Result<(), std::io::Error> {
        let mut handle = File::create(filename)?;

        let serialized = ser::to_bytes(torrent).unwrap();
        handle.write_all(&serialized)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
