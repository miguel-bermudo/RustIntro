mod customPointer;
mod otherSmartPointers;

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::ops::Deref;
use customPointer::{test_pointer, early_drop_call};
use otherSmartPointers::referenced_counted_sp;

#[derive(Debug)]
enum List{
    Cons(i32, Rc<List>),
    Nil,
}


// Building a custom smart pointer from Box:
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// we implement de deref trait so we can get the data inside the box with the asterisk operator.
// calling *Mybox returns the value.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    boxed();
    dereferencing();

    // deref coercion makes it so we can deref references into a reference to their original types when passing it.
    // for example we give a MyBox to the string Rust and pass it as a &str (it has been dereferenced to its base type automatically.)
    // since MyBox and String implement deref.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // without deref coercion it would look like this
    hello(&(*m)[..]);

    // there are some exceptions we don't explore here with derefMut

    test_pointer();
    early_drop_call();
    referenced_counted_sp();
}

fn boxed(){
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    println!("conned list. = {:#?}", list);
}

fn dereferencing(){
    let x = 5;
    let y = Box::new(x);

    if 5 == x && 5 == *y{
        println!("they are equal")
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}