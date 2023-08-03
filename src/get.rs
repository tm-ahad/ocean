use crate::object_dictionary::ObjectDictionary;

pub unsafe fn get(key: &String, objects: &ObjectDictionary) {
    objects.get(key)
        .unwrap_or_else(|| {
            println!("{key} not found");
            todo!()
        })
        .print();
}
