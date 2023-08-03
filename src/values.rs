use crate::catch::Cache;
use crate::dsp::Dsp;
use std::io::Read;

pub unsafe fn values() {
    let file =  Cache::read();

    match file {
        Some(file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .unwrap_or_else(|e| panic!("{e}"));

            let dsp = Dsp::new(content);

            let values = dsp.values();
            let mut log = String::new();

            for value in values {
                log.push_str(format!("{}\n", &value).as_str())
            }

            println!("{}", log);
        },
        None => println!("No opened file found")
    }
}
