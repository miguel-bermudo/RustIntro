struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
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

    //5.2

    
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