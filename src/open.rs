use crate::object_dictionary::ObjectDictionary;
use crate::type_and_object::Object;
use crate::catch::Cache;
use std::fs::OpenOptions;
use std::process::exit;
use std::io::Read;

pub unsafe fn open(args: Vec<String>, objects: &mut ObjectDictionary) {
    let p = &args[1];

    let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("/usr/ocean/{}.dsp", p))
            .unwrap_or_else(|e| {
                println!("{e}");
                exit(1);
                todo!()
            });
    Cache::set(file);

    let file = Cache::read()
        .unwrap_or_else(|| panic!("Not possible!"));

    let mut content = String::new();

    file.read_to_string(&mut content)
        .unwrap_or_else(|e| panic!("{e}"));

    for lin in content.lines() {
        let p = Object::from_string(lin.to_string());

        objects.insert(p.0, p.1);
    }
}