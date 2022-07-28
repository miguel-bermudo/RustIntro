mod generics;
mod traits;
mod lifetimes;

use generics::{Point, MixedPoint, largest};
use traits::{Summary, Tweet, NewsArticle, notify, notify_generic,
notify_two, notify_two_generics, some_function, some_function_where_clause};
use lifetimes::{lifetimes};


fn main() {
    largest_in_two_list_copied();
    largest_of_two_lists_reference();

    // Using the generic point we can use it to build float or i32 points in space.
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}, p.y = {}", p.x(), p.y());

    let f = Point { x: 1.0, y: 4.0 };
    println!("{}",f.distance_from_origin());

    // but not both...
    // let wont_work = Point { x: 5, y: 4.0 }; // <- this does not compile.

    // unless we specifically declare it as mixed "pub struct MixedPoint<T,U>"
    let mixed = MixedPoint { x: 1, y: 4.0 };

    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // 10.2
    aggregator();
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    
    //10.3
    lifetimes();
}

fn aggregator(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    notify(&tweet);
    notify_generic(&tweet);
    notify_two(&tweet, &article);
    // Doesn't work curz tweet and article are no the same type.
    // notify_two_generics(&tweet, &article)
    // some_function(t, u)
    // some_function_where_clause(t, u)
    let (something, another_something) = returns_summarizables();
    // println!("{}, {}", somethings, another_something);
}

fn returns_summarizables() -> (impl Summary, impl Summary) {
    // we can specify that we are returning something that implements Summary. (could be tweet or article)
    let bruh = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let man = NewsArticle{
        headline:String::from("broski dies"),
        author:String::from("broski"),
        content:String::from("Author that wrote this dies, this is a paradox."),
        location:String::from("broski's living room"),
    };

    (bruh, man)
}

fn largest_of_two_lists_reference(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}, and the list is unmutated {:?}", result, number_list);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn largest_in_two_list_copied(){
    let mut number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in &mut number_list {
        if number > &mut largest {
            largest = *number;
        }
    }
    // This approach destroys the original list.
    println!("The largest number is {}, and the list is unmutated {:?}", largest, number_list);

    let mut number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];

    for number in &mut number_list {
        if number > &mut largest {
            largest = *number;
        }
    }

    println!("The largest number is {}", largest);
}