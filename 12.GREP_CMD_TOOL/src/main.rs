use std::{env, fs};

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {

        if args.len() != 3 {
            panic!("you must specify a word and a text file.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

fn main() {
    // gets console line arguments.
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}\n", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}\n", contents);
}