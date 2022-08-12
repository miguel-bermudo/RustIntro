mod customPointer;
mod otherSmartPointers;

use crate::List::{Cons, Nil};
use crate::RefList::{RefCons, RefNil};
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use customPointer::{test_pointer, early_drop_call};
use otherSmartPointers::{referenced_counted_sp};

#[derive(Debug)]
enum List{
    Cons(i32, Rc<List>),
    Nil,
}


#[derive(Debug)]
enum RefList{
    RefCons(Rc<RefCell<i32>>, Rc<RefList>),
    RefNil,
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
    refcells_with_rc();
    reference_cycle();
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

fn refcells_with_rc(){
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RefCons(Rc::clone(&value), Rc::new(RefNil)));

    let b = RefCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // the modification applies to all the values, effectively mutating a variable borrowed by various Con lists.
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn reference_cycle(){
    // this is a memory leak, aka a fucky wucky.
    #[derive(Debug)]
    enum aList {
        aCons(i32, RefCell<Rc<aList>>),
        aNil,
    }

    use aList::{aCons, aNil};

    impl aList {
        fn tail(&self) -> Option<&RefCell<Rc<aList>>> {
            match self {
                aCons(_, item) => Some(item),
                aNil => None,
            }
        }
    }

    let a = Rc::new(aCons(5, RefCell::new(Rc::new(aNil))));

    println!("\n a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?} \n", a.tail());

    let b = Rc::new(aCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?} \n", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

}