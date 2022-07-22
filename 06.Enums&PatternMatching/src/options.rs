#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            if rand::random(){
                println!("Lucky penny!");   
            }
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quarter from {:?}!", state);
            25
        } ,
    }
}

pub fn testOptions(){
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.expect(&String::from("bruh")); // panics with error message if none
    let sum = x + y.unwrap();// hardcore panics if option is none
    //6.2

    let c = Coin::Quarter(UsState::Alaska);
    value_in_cents(&c);
    
    let c1 = Coin::Penny;
    value_in_cents(&c1);

}