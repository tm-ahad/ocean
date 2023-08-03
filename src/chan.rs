use std::fs::read_to_string;

pub fn chan() {
    let p = read_to_string("/usr/ocean/chan.ss")
        .unwrap_or_else(|e| panic!("{e}"));
    println!("{p}")
}