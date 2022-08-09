mod processing_with_iterator;


use std::thread;
use std::time::Duration;

use processing_with_iterator::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

    
}

fn main() {
    invent();
    closures();
    rectangles();
    iterators();
}

fn invent(){
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn closures(){
    // this is an expensive closure
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    
    // function vs closures...
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    
    // these are all valid ways to declare it.
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // if you use closures without explicitely indicating the variable, you must call them at least once or they wont compile.
    let add_one_v3 = |x| { x + 1 };
    add_one_v3(5);
    let add_one_v4 = |x|   x + 1 ;
    add_one_v4(3);


    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5.to_string());

    // closure gets their variables in 3 ways
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling mutable closure: {:?}", list);
}

fn rectangles() {
    // closures are useful to define sorters for example.
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
        Rectangle {
            width: 15,
            height: 12,
        },
    ];
    
    let mut num_sort_operations = 0;
    // this sorts the list by the width of the rectangle and increases the counter for sort operations.
    // we can then see the number of sort operations done by rust when using this sort by key method.
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
