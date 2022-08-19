use std::sync::{Mutex, Arc};
use std::thread;

pub fn scopes(){
    // by using locks you can share data with rust mutex, it handles the lokcing and unlocking of the data so you don't have to.
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

pub fn thread_sharing(){
    // we make counter a rc that is atomic (Arc), this is sowhen the variable is shareable between threads.
    // let counter = Rc::new(Mutex::new(0)); <-Rc variant, non thead safe.
    let counter = Arc::new(Mutex::new(0)); // <- thread safe variant with Arc

    let mut handles = vec![];

    for _ in 0..10 {
        // we need to make a clone that references the counter variable.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    // we unwrap all the threads
    for handle in handles {
        handle.join().unwrap();
    }

    // finally we have the result of having 10 threads all sum 1 to the variable.
    println!("Result: {}", *counter.lock().unwrap());

    // Note: the std:: atocmic library has a multitude of types that work to perform thread safe opertations with primitive types
    // Mutex is a generalist type the stablish a pararlel between RefCell/Rc and Mutex/Arc for threads.
}

pub fn deadlock() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let handle = thread::spawn(move || {
            let mut a_num = a.lock().unwrap();
            *a_num += 1;
            println!("Thread 1 holds a lock and starts waiting b lock");
            let mut b_num = b.lock().unwrap();
            *b_num += 1;
        });
        handles.push(handle);
    }
    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let handle = thread::spawn(move || {
            let mut b_num = b.lock().unwrap();
            *b_num += 1;
            println!("Thread 2 holds b lock and starts waiting a lock");
            let mut a_num = a.lock().unwrap();
            *a_num += 1;
            println!("Thread 2");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Done {}", *a.lock().unwrap()); // never reach here
}