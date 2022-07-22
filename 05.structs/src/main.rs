

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn width(&self) -> bool { // we can have methods with the same name as variables.
        self.width > 0
    }
}
//we can also have multiple Impl blocks.
impl Rectangle{
    fn can_hold(&self, r: &Rectangle) -> bool{
        self.width > r.width && self.height > r.height
    }

    fn square(size:u32)->Rectangle{
        Rectangle { width:size, height: size}
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-like struct
struct AlwaysEqual;


fn main() {
    // 5.1
    let u = build_user(String::from("someone@example.com"), String::from("someusername123"));
    print_user(&u);

    //copying from a struct
    let new_user = User{
        active: u.active,
        username: u.username.clone(), // as the string would move we need to clone it.
        email: String::from("another@example.com"),
        sign_in_count: u.sign_in_count,
    };

    print_user(&new_user);

    // If you want to copy from an struct to one of the same type you can modify the fields u want to and 
    // add ..<struct to copy from> to the bottom as a shorthand
    let new_user1 = User{
        email: String::from("another@example.com"),
        ..u
    };
    print_user(&new_user1);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    //5.2 & 5.3
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(20);

    println!("rect1 is {:?}", rect1); //debug print.
    println!("rect1 is {:#?}", rect1); //pretty debug print
    dbg!(&rect1); // another debug print.

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold sq? {}", rect1.can_hold(&sq));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

fn print_user(u: &User){
    println!(" mail: {}\n username: {}\n is_active: {}\n sign_ins: {}\n", u.email, u.username, u.active, u.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username, // you can either declare the variable to reassign explicitely
        //or use the shorthand by naming the variable passed as the same name to the assignement in the function.
        active: true,
        sign_in_count: 1,
    }
}