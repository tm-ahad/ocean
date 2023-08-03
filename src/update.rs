use crate::clear::clear;
use crate::object::object;
use crate::object_dictionary::ObjectDictionary;

pub unsafe fn update(lin: String, obj: &mut ObjectDictionary) {
    let spl = lin.split(" ").collect::<Vec<&str>>();

    clear(&spl[1].to_string());
    object(lin, obj);
}