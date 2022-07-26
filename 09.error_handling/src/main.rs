use std::{env, fs::File, fs, cmp::Ordering};
use std::io::{self, ErrorKind, Error, Read};
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // this method needs to be inside main() method
    env::set_var("RUST_BACKTRACE", "1"); // -> detailed error messages
    // unrecoverable_panic();
  
    recoverable_errors();
    propagating_errors();

    guessing_game_with_checks();
    guessing_game_with_custom_struct(); // this one panics thats bad.
}

fn guessing_game_with_custom_struct(){
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {println!("That is not a valid number, try again"); continue},
        };

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("The number is bigger..."),
            Ordering::Greater => println!("The number is smaller..."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn guessing_game_with_checks(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("That is not a valid number, try again"); continue},
        };

        if !(1..=100).contains(&guess) { // alias of !(1..=100).contains(&guess)
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is bigger..."),
            Ordering::Greater => println!("The number is smaller..."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn propagating_errors(){
    let mut res = read_username_from_file(); // propagates the result upwards
    res = read_username_from_file_short();
    res = read_username_from_file_shortest();

    // but all of this can actually be acomplished just by calling this:
    res = fs::read_to_string("hello.txt");

    // the ? operator can only be used inside functions that return a Result, Option or another type that implements FromResidual.
    // it couldn't be used inside of the main function for example.
    // res = fs::read_to_string("hello.txt")?;

    last_char_of_first_line("niggerfaggot");
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_username_from_file_shortest() -> Result<String, io::Error>{
    let mut s = String::new();
    // we can even concatenate the ? operator with the next operation.
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_short() -> Result<String, io::Error>{
    // does the same as the other function but without being so verbose.
    // the ? operator here gets the result and returns the error if it catches it
    // otherwise it assigns the value inside the result to the othe side of the expression.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
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