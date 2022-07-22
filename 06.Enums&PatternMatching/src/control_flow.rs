#[derive(Debug)]
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

pub fn control_flow(){
    if_let_vs_match();
    let c = Coin::Quarter(UsState::Alaska);
    if_let_else_vs_match(&c)
}

fn if_let_vs_match(){
    let config_max = Some(3u8);// 3u8 is just a 3 
    // these two snippets do the same, but match offers more flexibility, 
    // "if let" is a sugar for match this and ignore the rest of the values.
    match config_max {
        Some(variable_name) => println!("The maximum is configured to be {}", variable_name),
        _ => (),
    }

    if let Some(variable_name) = config_max {
        println!("The maximum is configured to be {}", variable_name);
    }
}

fn if_let_else_vs_match(c:&Coin){
    // if let can also have an else and be the same that running code in the _ part of the match.

    let mut count = 0;
    match c {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    count = 0;
    if let Coin::Quarter(state) = c {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}