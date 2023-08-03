use std::process::exit;
use crate::object_dictionary::KeyObjectPair;

pub enum Type {
    String,
    U8
}

#[derive(Debug)]
pub enum Object {
    String(String),
    U8(u8),
}

impl Object {
    pub fn from_str(typ: &str, val: &str) -> Object {
        match Type::from_str(typ) {
            Type::String => Object::String(val.to_string()),
            Type::U8 => Object::U8(val.parse::<u8>()
                .unwrap_or_else(|_| {
                    println!("Invalid u8");
                    todo!()
                }))
        }
    }
}

impl Type {
    pub fn from_str(s: &str) -> Type {
        match s.trim() {
            "String" => Type::String,
            "u8" => Type::U8,
            _ => {
                println!("Invalid type");
                exit(1);
                todo!()
            }
        }
    }
}

impl Object {
    pub fn print(&self) {
        match self {
            Object::String(s) => {
                println!("String:{}", s)
            },
            Object::U8(u) => {
                println!("u8:{}", u)
            },
        };
    }

    pub fn from_string(o: String) -> KeyObjectPair {
        let s = o.split(":").collect::<Vec<&str>>();

        let key = s[0].to_string();
        let val = s[1];
        let typ = s[2];

        let obj = Object::from_str(typ, val);

        KeyObjectPair(key, obj)
    }
}