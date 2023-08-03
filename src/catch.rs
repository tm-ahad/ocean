use std::fs::File;

static mut FILE: Option<File> = None;

pub struct Cache;

impl Cache {
    pub unsafe fn read<'a>() -> Option<&'a mut File> {
        match &mut FILE {
            Some(a) => Some(a),
            None => None
        }
    }

    pub unsafe fn set(val: File) {
        FILE = Some(val);
    }
}