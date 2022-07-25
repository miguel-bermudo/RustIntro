use std::collections::HashMap;

mod suggested_exercises;
use suggested_exercises::{exercises};

fn main() {
    // 8.1
    list_of_values_with_vec();
    // 8.2
    utf_8_text_with_string();
    indexing_with_strings(); 
    //8.3
    hashmaps();
    // exercises:
    exercises();
}

fn hashmaps(){
    // manual insert
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    // zip two vecs into hasmap with collect.
    let teams = vec!["Blue", "Yellow"];
    let initial_scores = vec![10, 50];

    // we need to specify hasmap as collect can do so into many types.
    // using underscores so rust infers the type.
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // hasmaps take ownership of wnything that does not implement the copy trait.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // println!("{}", field_name) // -> moved error.

    // getting values out.
    let score = scores.get("Blue");

    // in case its not &str and its string.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);// <- pass a reference not the value itself, it returns option.
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwrite a value.
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // inserting only if value does not exist.
    // entry also returns a mutable reference to the value if it does insert it.
    scores.entry(String::from("Green")).or_insert(75); // inserts green
    scores.entry(String::from("Blue")).or_insert(50); //does nothing, blue stays at 25

    println!("{:?}", scores);

    // updating a value based on the old value.
    // get entry and if it exists updates the counter value to be += 1
    // otherwise inserts 0 then updates it to 1.

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);


}

fn indexing_with_strings(){
    let s1 = String::from("hello");
    // let h = s1[0]; -> string cannot be indexed directly
    // this is because strings are a vec of characters, and that can be a number of weird hindi shit and japanese
    // they ruin good thigs for the rest of us ~.~

    // we can either collect the characters from the s1.chars() iterator or can use nth to get directly an Option, its better to use nth in this case.
    let chars: Vec<char> = s1.chars().collect();
    let h = chars[0];
    let e = s1.chars().nth(1);
    println!("{}{}",h,e.unwrap());

    // you can get a reference to a substring tho, (which in all effects is almost the same.)
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}",s); // this returns "Зд" cuz cyricil characters takes 2 bytes.

    //let s = &hello[0..1]; // this panics cuz 1 byte is not a valid string.

    // slicing string is dangerous as fuck, thats why its better to iterate over strings:
    for c in "नमस्ते".chars() {
        // this returs the explicit valid characters inside the string न म स त (two accents don't show up)
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        // here we can see the byte values that make up the string: 224,164,168,...,165,135
        println!("{}", b);
    }
}

fn utf_8_text_with_string(){
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string(); // <- adding the to_string converts it into a string while...
    let s = "broski"; // <- this is a string literal.
    // to_string can also be used on literals directly
    //and acts the same as Strin::from()
    let s = "broski".to_string();
    // it can also handle any UTF-8 text.
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let mut hello = String::from("Hola");
    // you can even concatenate text with the + operator:
    hello += " bruh";
    
    //or the push_str
    hello.push_str(", we go jim");
    println!("{}",hello);

    //or the push method that appends a single character
    hello.push('?');
    println!("{}",hello);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //these two do the same but format is better for this kind of stuff. (f-string equivalent.)
    let s = format!("{}-{}-{}", s1, s2, s3); // this doesn't take ownership of anything so its keeps the references intact
    println!("{}",s);
    let s = s1 + "-" + &s2 + "-" + &s3; // s1 is borrowed to s so its out of scope after here.
    println!("{}",s);
}

fn list_of_values_with_vec(){
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v = Vec::new();
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // Panic, cuz element does not exist, 
    // let does_not_exist = &v[100];
    
    // using get does not panic, return none instead.
    let does_not_exist = v.get(100);

    // borrow checker bs
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // since we get a reference to the vector (first), we cannot modify it afterwards, (v.push(6)), cuz rust rules.
    // v.push(6);
    println!("The first element is: {}", first);

    for i in &mut v{
        // to get a reference to a value in the heap we need to use the * deference operator.
        *i += 50;
        println!("{}", i);
    }


    // To store different values in a vec we first create an enum with all the values we need to store.
    // then we instantiate each enum with one of the value types.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}