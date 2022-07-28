use::std::fmt::{Display, Debug, Result, Formatter};

pub trait Summary { 
    // traits are like the common implementation sigantures
    // then we user impl declarations for a struct
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// if we don't implement the summarize trait in the impl block
// it takes the default we implemented in the trait.
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// this allows us to print it.
impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(f, "{}{}{}{}", self.headline, self.location, self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// this allows us to print it.
impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(f, "{}{}{}{}", self.username, self.content, self.reply, self.content)
    }
}

// We can specify the type directly or...
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// specify a generic type that implements the Summary trait.
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify_two_generics<T: Summary>(item1: &T, item2: &T) {}

// using where clauses to define generic trait boundaries
// this:
pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {0}

// to this:
pub fn some_function_where_clause<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone, U: Clone + Debug
{0}

// Using trait bounds to conditionally Implement Methods

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
