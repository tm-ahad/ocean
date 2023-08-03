use std::collections::HashMap;
use crate::type_and_object::Object;

#[derive(Debug)]
pub struct  KeyObjectPair(pub String, pub Object);
pub type ObjectDictionary = HashMap<String, Object>;
