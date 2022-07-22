fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn matching_options(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // _ => reroll(), runs code on any other possibility
        _ => (), //ignores every other posibility
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}