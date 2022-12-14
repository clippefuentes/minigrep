use std::env;
use std::process;

use minigrep::Config;
use minigrep:: run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
       println!("Problem with Parsing: {}", err);
       process::exit(1); 
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}

