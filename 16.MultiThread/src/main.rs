mod shared_state;

use std::iter::Rev;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use shared_state::*;


fn main() {
    join_ops();
    move_threads();
    // message_passing();
    // multiple_trans();

    //Shared variable with locks.
    shared_state::scopes();
    shared_state::thread_sharing();

    shared_state::deadlock();
}

fn join_ops(){
    // we use thread::spawn to create a new thread, and we pass it a closure
    // (javascrip like temporary function.)
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // spawned thread dies here, so if they do the same, the thread dies before completing its task.

    // unless we use the join operation to rejoin it to the main thread
    // join is blocking meaninig it waits for the thread to finish before going on.
    handle.join().unwrap();

    // you can also use join to make the thread finish whener u want.
}

fn move_threads(){
    let v = vec![1, 2, 3];

    // since v's value can dissapear before the thread ends we need to add the move keywor before the closure.
    // this forces the closure function to take ownership of the variable v.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn message_passing(){
    // we create a channel in the main thread which would be the reciever thread.
    let (tx, rx) = mpsc::channel();

    // we create a new thread and move tx (transmiter) ownership into it.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // using the value after send results in an error since send uses the value up.
        // println!("vals is {:?}", vals);
    });

    // the reciever which is still in the main thread, gets the message back from the thread.
    // recieving is a blocking operation which means the thread will block untill the other one sends the message.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // alternatively we can use try_recieve which just gets a message if its avaliable and an Err if not. (Result<T,E>)
    // its usefull to do some work if we do not have messages and then try again in a loop.
    loop{
        let received = rx.try_recv().unwrap_or_else(|_| "no message recieved.".to_string());
        thread::sleep(Duration::from_millis(100));
        println!("waiting for message.");
        if received != "no message recieved." {
            println!("Got: {}", received);
            // do something if we recieve a message and exit.
            break;
        }
    }

    for received in rx {
        // we can recieve the rest of the messages like this
        // we are using rx as an iterator which is very usefull cuz it recieves data dinamically.
        println!("Got: {}", received);
    }
}

fn multiple_trans(){
    // Creating multiple Producers by clonning the transmiter
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}