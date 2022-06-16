
pub fn mutability(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The new value is: {}", x);
}

pub fn constants(hours: &u32) {
    const HOURS_IN_SECONDS: u32 = 60*60;
    println!("{} hour(s) in seconds is: {}", hours, hours*HOURS_IN_SECONDS);
}

/// mantains the inmutability of a variable while performing changes to it.
/// 
/// you can even change the type of the vaible while maintaining the name.
/// 
/// something you cannot otherwise do to variables declared with mut.
pub fn shadowing_and_scopes(){
    let x = 5;
    println!("The value of x before shadowing is: {}", x);

    let x = x+1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    let x = x.to_string();
    let x = x+"4";
    println!("The value of x outside the inner scope is: {}, but its now a string.", x);
}