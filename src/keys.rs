use crate::catch::Cache;
use crate::dsp::Dsp;
use std::io::Read;

pub unsafe fn keys() {
    let file =  Cache::read();

    match file {
        Some(file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .unwrap_or_else(|e| panic!("{e}"));

            let dsp = Dsp::new(content);

            let mut log = String::new();
            let keys = dsp.keys();

            for key in keys {
                log.push_str(format!("{}\n", &key).as_str())
            }

            println!("{}", log);
        },
        None => println!("No opened file found")
    }
}

