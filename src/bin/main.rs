use std::env;
use std::process;

use kasm::errors::Error;
use kasm::run;
use kasm::usage;
use kasm::Config;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let config = match Config::new(&mut args) {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err.1);
            usage(&args[0]);
            process::exit(err.0 as i32);
        }
    };

    match run(&config) {
        Ok(_) => {
            process::exit(Error::NoError as i32);
        }
        Err(err) => {
            println!("{}", err.1);
            process::exit(err.0 as i32);
        }
    }
}
