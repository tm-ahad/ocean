use crate::catch::Cache;
use crate::object_dictionary::ObjectDictionary;
use crate::type_and_object::{Object, Type};
use std::io::{Seek, SeekFrom, Write};

pub unsafe fn object(lin: String, objects: &mut ObjectDictionary) {
    let toks = lin.split(" ").collect::<Vec<&str>>();

    let lp = toks[1].split(":").collect::<Vec<&str>>();
    let name = lp[0].to_string();
    let typ = lp[1];
    let val = toks[3].to_string();

    if toks[2] == "=" {
        let pv = match Type::from_str(typ) {
            Type::String => Object::String(val.to_string()),
            Type::U8 => Object::U8(val.parse::<u8>()
                .unwrap_or_else(|_| panic!("Unvalid u8 value"))
            ),
        };

        objects.insert(name.clone(), pv);
    }

    let file = Cache::read();

    match file {
        Some(file) => {
            file.write_all(format!("{}:{}:{}\n", name, val, typ).as_bytes())
                .unwrap_or_else(|e| panic!("{e}"));

            file.seek(SeekFrom::Start(0))
                .unwrap_or_else(|e| panic!("{e}"));
        },
        None => println!("No opened file found")
    }
}