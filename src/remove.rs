use std::fs::{OpenOptions, remove_file, read_to_string};
use std::io::{Read, Write};

pub fn remove(name: &String) {
    match name.as_str() {
        "*" => {
            let chan = read_to_string("/usr/ocean/chan.ss")
                .unwrap_or_else(|e| panic!("{e}"));

            let split = chan.split('\n');

            for lin in split {
                match lin {
                    "" => {},
                    _ => remove(&lin.to_string())
                }
            }
        }
        _ => {
            remove_file(format!("/usr/ocean/{name}.dsp"))
                .unwrap_or_else(|e| panic!("{e}"));

            let mut chan = OpenOptions::new()
                .read(true)
                .write(true)
                .open("/usr/ocean/chan.ss")
                .unwrap_or_else(|e| panic!("{e}"));

            let mut content = String::new();
            chan.read_to_string(&mut content)
                .unwrap_or_else(|e| panic!("{e}"));

            match content.find(name) {
                Some(i) => content.replace_range(i..i+name.len(), ""),
                None => println!("Channel {name} not found")
            }

            chan.write_all(content.as_bytes())
                .unwrap_or_else(|e| panic!("{e}"));
        }
    }


}