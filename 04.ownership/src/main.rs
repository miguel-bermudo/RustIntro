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
    let x = 5;
    let y = x;
    println!("{}{}",x,y);

    // since ultimatelly stings are lists, they are of mutable size,
    // and this makes it so its not borrowable if inmutable
    // for this reason strings do not have the move operation.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}{}",s1,s2)
}

fn main() {
    scopes();
    hello_world();
    move_var();
}