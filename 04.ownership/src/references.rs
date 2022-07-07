pub fn change(some_string: &String){
    //some_string.push_str(", wor&ld"); // you cannot change borrowed variables
}

pub fn change_mut(some_string: &mut String) -> &mut String{
    some_string.push_str(", world!");
    return some_string
}

pub fn multi_borrow(){
    let mut s = String::from("hello");

    // you cannot borrow multiple variables at the same time
    {
        // you can change scope and borrow it in that scope 
        let r2 = &mut s;
        println!("{}", r2);

    }

    //before borrowing it again in another scope.
    let r1 = &mut s;
    println!("{}", r1);
}

pub fn mutability_comb(){
    // mutability cannot be mutable and inmutable at the same time.
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
}

pub fn mut_borrow_correct(){
    // since the compiler knows when its the last time u use a variable in a 
    // certain scope it can determine wether or not you can do a mutable copy of a variable
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

