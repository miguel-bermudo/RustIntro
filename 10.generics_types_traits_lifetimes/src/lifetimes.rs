use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // both of these cases are covered by the ellision rules.
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn lifetimes(){
    // static lifetime parameters live for all of the lifetime of the program.
    let s: &'static str = "I have a static lifetime.";

    {
        let x = 5;
        let r = &x; // <-- this is in different scopes so borrow checker is an asshole.
        println!("r: {}", r);
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Since we added the lifetime params to the function we can now declare string out of scope
    // and use it in the next scope.
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Lifetims in struct definitions.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    i.announce_and_return_part("example announcement");
    i.level();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // lifetime parameters are a way to indicate that the recieved parameters must live as long as any of the lifetime parameters do.
    // so for example if you recieve two parameters with lifetimes here and return one with the 
    // same lifetime reference they must continue living beyond the scope of this function and until the returned one does.

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ALL COMBINED!
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}