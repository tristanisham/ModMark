use std::env;
use std::process;

use modmark::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

// consider removinng third argument [2]
    println!("Converting '{}' to {}", config.conv_from, config.conv_to);

    if let Err(e) = modmark::run(config) {

        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}


    
