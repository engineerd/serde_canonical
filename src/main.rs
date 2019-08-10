extern crate serde_canonical;
extern crate serde_json;

use std::{env, fs, path};

const ARG_PANIC_MESSAGE: &str =
    "The first argument is the input JSON, and the second is an optional output file.";

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input = fs::File::open(path::Path::new(input)).expect("cannot open input file");
    let res: serde_json::Value =
        serde_json::from_reader(input).expect("cannot deserialize input file");
    match args.len() {
        2 => {
            print!(
                "{}",
                serde_canonical::ser::to_string(&res).expect("cannot write canonical JSON")
            );
        }
        3 => {
            let output = &args[2];
            let mut output = fs::File::create(path::Path::new(output))
                .expect("cannot create or open output file");
            serde_canonical::ser::to_writer(&mut output, &res)
                .expect("cannot write canonical JSON");
        }
        _ => panic!(ARG_PANIC_MESSAGE),
    };
}
