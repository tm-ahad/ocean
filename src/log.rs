use crate::catch::Cache;
use std::io::Read;

pub unsafe fn log() {
    let f = Cache::read()
        .unwrap_or_else(|| panic!("No opened db found"));

    let mut cont = String::new();
    f.read_to_string(&mut cont)
        .unwrap_or_else(|e| panic!("{e}"));

    println!("{}", cont);
}