use std::fs::File;
use std::io::{stdout, Write};

pub fn init() {
    let mut shf = File::create("./ocean_init.sh")
        .unwrap_or_else(|e| panic!("{e}"));

    let cont = String::from("\
sudo mkdir /usr/ocean
sudo chmod 777 /usr/ocean
touch /usr/ocean/chan.ss
    ");

    shf.write_all(cont.as_bytes())
        .unwrap_or_else(|e| panic!("{e}"));

    let msg = String::from("To initalize ocean run the ./ocean_init.sh script\n");

    stdout()
        .write_all(msg.as_bytes())
        .unwrap_or_else(|e| panic!("{e}"));
}