use std::{collections::HashMap, fs};

use brs::torrent::v1;

pub(crate) fn metadata(v1: bool, _v2: bool, path: String) {
    if v1 {
        let bytes = fs::read(path).unwrap();
        let torrent = v1::Torrent::parse_bytes(&bytes);
        if let Err(e) = &torrent {
            eprintln!("{e}");
        }
        println!("{}", torrent.unwrap())
    } else {
        unimplemented!()
    }
}

pub(crate) fn raw(path: String) {
    let bytes = fs::read(path).unwrap();
    let out: HashMap<String, bendy::value::Value> = bendy::serde::from_bytes(&bytes).unwrap();
    println!("{:?}", out)
}

pub(crate) fn create(_path: String, _data: String) {
    unimplemented!()
}
