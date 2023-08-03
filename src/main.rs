mod new;
mod open;
mod catch;
mod init;
mod get;
mod dsp;
mod object;
mod clear;
mod chan;
mod remove;
mod update;
mod keys;
mod exec;
mod values;
mod log;
mod type_and_object;
mod object_dictionary;

use crate::exec::exec;
use std::env::args;
use std::fs::read_to_string;
use crate::object_dictionary::ObjectDictionary;

fn main() {
    let mut args = args();
    let _ = args.next();
    let mut objcects = ObjectDictionary::new();

    match args.next() {
        Some(s) => unsafe {
            let content = read_to_string(s)
                .unwrap_or_else(|e| panic!("{e}"));

            let split = content.split('\n');

            for lin in split {
                exec(lin.to_string(), &mut objcects)
            }
        },
        None => println!("ocean 1.0.0")
    }
}