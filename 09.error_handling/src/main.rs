use std::{env, fs::File};
use std::io::{self, ErrorKind, Error, Read};

fn main() {
    // this method needs to be inside main() method
    env::set_var("RUST_BACKTRACE", "1"); // -> detailed error messages
    // unrecoverable_panic();
  
    recoverable_errors();
    propagating_errors();
}

fn propagating_errors(){
    let res = read_username_from_file(); // propagates the result upwards
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn recoverable_errors(){
    let f = File::open("hello.txt");
    let file = handling_with_closures(f);

    let f = File::open("hello.txt");
    let file = handling_with_match(f);
    
}

fn handling_with_closures(f : Result< File, Error>) -> File {
    let f = f.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    f
}

fn handling_with_match(f : Result< File, Error>) -> File {
    println!("{:?}", f); // -> here f is a Result type
    
    // handling results type
    let f = match f {
        Ok(file) => file,
        // matchign multiple error types
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("{:?}", f); // here f is the handle for the file.
    f
}

fn unrecoverable_panic(){
    // direct crash
    panic!("crash and burn");

    // out of range
    let v = vec![1, 2, 3];
    v[99]; // <- this is out of range of the array.
}