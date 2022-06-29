#![allow(unused)]

pub fn scopes(){
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    // println!("{}", s) <- this would be out of scope and wouldn't print anything.
    // the compiler will also free the memory in the heap that belongs to s, since it went out of scope.
}

fn hello_world(){
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}

fn move_var(){
    // this first example works because integers are simple values that can be saved fully in the stack
    // all the values that are on the stack directly are: bool, char
    // All the integer types, such as u32, All the floating point types, such as f64
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    let x = 5;
    let y = x;
    println!("{}{}",x,y);

    let z = true;
    let w = z;
    println!("{}{}",z,w);

    // since ultimatelly stings are lists, they are of mutable size,
    // and this makes it so its not borrowable if inmutable
    // for this reason strings do not have the move operation.
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}{}",s1,s2) // -> this doesn't compile.
}

fn clone_var(){
    // we use the common function clone to copy values from one variable to another
    //in case the variable is not a primitive type.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn gives_ownership() -> String {             
    // gives_ownership will move its return value into the function that calls it

    // some_string comes into scope
    let some_string = String::from("yours"); 

    // some_string is returned and moves out to the calling function
    some_string                              
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { 
// a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn main() {
    scopes();
    hello_world();
    move_var();
    clone_var();


    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
    //println!("{}",s);             // ... and so is no longer valid here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
    println!("{}",x);               // but i32 is Copy, so it's okay to still// use x afterward



    // gives_ownership moves its return value into s1
    let s1 = gives_ownership();
    // s2 comes into scope
    let s2 = String::from("hello");  
    // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);  

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}