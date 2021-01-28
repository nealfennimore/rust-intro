use std::fs::File;
use std::io::prelude::*;

// Project dependencies
use crate::utils;
use crate::cool;

pub struct FileDebug;

impl FileDebug {
    pub fn open(self) {
        let mut file = File::open("cool.txt").unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let cool = cool::AddrSpace::CoolNess(cool::CoolNess(
            "A".to_ascii_lowercase(),
            "B".to_ascii_uppercase(),
            "C".to_uppercase(),
        ));

        utils::logger::debug(buf);
        utils::logger::debug(cool);
    }
}