use crate::catch::Cache;
use std::io::{Read, Write};

pub unsafe fn clear(key: &String) {
    match key.as_str() {
        "*" => {
            let file =  Cache::read()
                .unwrap_or_else(|| panic!("No opened db not found"));

            file.set_len(0)
                .unwrap_or_else(|e| panic!("{e}"));
        },
        _ => {
            let file =  Cache::read();

            match file {
                Some(file) => {
                    let mut pre_cont = String::new();
                    file.read_to_string(&mut pre_cont)
                        .unwrap_or_else(|e| panic!("{e}"));

                    match pre_cont.find(key) {
                        Some(i) => {
                            let mut n = i;

                            while &pre_cont[n..n+1] != "\n" {
                                n += 1;
                            }

                            pre_cont.replace_range(i..n+1, "");
                        },
                        None => println!("{key} not found")
                    }

                    file.write_all(pre_cont.as_bytes())
                        .unwrap_or_else(|e| panic!("{e}"));
                },
                None => println!("No opened file found")
            }
        }
    }
}