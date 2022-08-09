use crate::List::{Cons, Nil};
use std::rc::Rc;


pub fn referenced_counted_sp(){
    // reference counting is used so a value can have multiple owners
    // it only deallocs when no references to the object are active.
    // Rc<T> is only for single thread applications, not for multi threading.

    // two cons list that share a third one.
    // does not compile cuz value is moved in the first assignement if we use Box<T>
    // when changing it to Rc<T> we can created the list that is referenced by the other two,
    // a is referenced by c and b
    // using Rc::clone does not create a deep copy of the lists as a.clone() would. so its faster.
    // we use strong_count to check the number of references to a.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}   

pub fn refCells_for_mutability(){
    // refcells bend the rules of rust by allowing us to mutate variables that have inmutable references.
    // similar to box<T> it also holds a single owner but it allows you to mutate references to inmutable variables.
    // RefCell is ONLY for single threaded programs.

    // Interior mutability.
    
}