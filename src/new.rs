use std::fs::OpenOptions;
use std::io::{Read, Write};

pub fn new(name: &String) {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("/usr/ocean/{}.dsp", name))
        .unwrap_or_else(|e| panic!("{e}"));

    let mut list = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open("/usr/ocean/chan.ss")
        .unwrap_or_else(|e| panic!("{e}"));

    let mut lis = String::new();
    list.read_to_string(&mut lis)
        .unwrap_or_else(|e| panic!("{e}"));

    match lis.find(name) {
        Some(_) => panic!("Channel {name} already found"),
        None => list.write_all(format!("{name}\n").as_bytes())
            .unwrap_or_else(|e| panic!("{e}"))
    }
}