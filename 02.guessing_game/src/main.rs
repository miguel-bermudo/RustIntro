use std::io;
// use std::process::exit;
use rand::prelude::*;
use std::cmp::Ordering::*;

// fn oh_shit() -> i32 {
//     return return return return return return return!!!!!!!!!!111111111
// }

fn main() {

    // let a = oh_shit();
    // println!("{}",a);
    // exit(0);

    println!("Guess the number from 1-100");
    let mut rng = rand::thread_rng();
    let to_guess = rng.gen_range(0..=100);
    let mut cont = 0;
    
    loop{
        println!("Please input your guess.");
        let mut guess_s = String::new();
        io::stdin().read_line(&mut guess_s).expect("failed to read line");

        // unwrap extracts the value from result, expect prints string in case of error and terminates.
        //let guess: u32 = guess_s.trim().parse().expect("Please type a number!"); 
        
        // if u want to replace the value of the variable wiht another in case of error
        let guess: u32 = match guess_s.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you didn't type a number, try again.");
                continue;
            }
        }; 

        match guess.cmp(&to_guess) {
            Less => println!("try a BIGGER number"),
            Greater => println!("try a SMALLER number"),
            Equal => {
                println!("you guessed correctly in {} tries", cont);
                break;
            },
        };

        cont += 1;


        // Lo mismo que en el mathc pero con if else encadenado:
        // if guess > to_guess{
        //     println!("your number is too LARGE, try again!");
        // }
        // else if guess < to_guess {
        //     println!("your number is too SMALL, try again!");
        // }
        // else{
        //     println!("you guessed correctly, it was {}", guess);
        //     break;
        // }
    }
}
