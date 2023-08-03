use crate::clear::clear;
use crate::get::get;
use crate::new::new;
use crate::remove::remove;
use crate::chan::chan;
use crate::init::init;
use crate::open::open;
use crate::object::object;
use crate::update::update;
use crate::keys::keys;
use crate::log::log;
use crate::object_dictionary::ObjectDictionary;
use crate::values::values;

pub unsafe fn exec(line: String, mor: &mut ObjectDictionary) {
    let or = &*mor;
    let args = line
        .split(' ')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    match args[0].as_str() {
        "remove"    => remove(&args[1]),
        "clear"     => clear(&args[1]),
        "new"       => new(&args[1]),
        "get"       => get(&args[1], or),
        "open"      => open(args, mor),
        "object"    => object(line, mor),
        "chan"      => chan(),
        "init"      => init(),
        "update"    => update(line, mor),
        "keys"      => keys(),
        "log"       => log(),
        "values"    => values(),
        ""          => {},
        _           => println!("Invalid command")
    }
}