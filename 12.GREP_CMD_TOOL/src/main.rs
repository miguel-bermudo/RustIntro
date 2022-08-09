use std::{env, process};
use GREP_CMD_TOOL::Config;

fn main() {
    // gets console line arguments.
    let args: Vec<String> = env::args().collect();
    
    // handles a mismatched number of arguments and exist with error code 1 
    // in case there is an error, assigns the values to confing otherwise.
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = GREP_CMD_TOOL::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}