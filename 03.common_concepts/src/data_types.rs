use std::io;

pub fn all(){
    /* NUMBER LITERALS
    All of them are u32 but are different type of number representations
    to see the number more clearly we can introduce an underscore(_) separating parts of it (usually thousands or bytes)
    */
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte= b'A';

    // printing them returns their value in decimal.
    println!("{}, {}, {}, {}, {}", decimal, hex, octal, binary, byte);


    //No unsigned for FLOATS only f64 and f32
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("{}, {}", x, y);

    // ARITHMETICS
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0, because integer division does not count remainder
    // remainder
    let remainder = 2 % 3;

    println!("{}, {}, {}, {}, {}, {}", sum, difference, product, quotient, floored, remainder);

    //BOOLEAN
    let t = true;
    let f: bool = false; // with explicit type annotation

    //CHAR
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{}, {}, {}, {}, {}", t, f, c, z, heart_eyed_cat);

    //COMPOUND TYPES (TUPLES & ARRAYS)
    //TUPLES
    // hard typing
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup: (i64, bool, &str) = (46, true, "pocholate");

    // soft typing 
    let tup = (123, "heyy", true, 1.5, [1,2,3]);

    // multi assigning (destructuring)
    let (x, y, z, m, n) = tup;  

    println!("{},{},{},{}", x,y,z,m);

    // we can also access the value at index of a tuple with a dot.
    let lol = tup.1;

    println!("{}, {}", lol, y);

    //ARRAYS
    // fixed in size when declared, vectors are not.
    // soft declaration
    let a = [1, 2, 3, 4, 5];
    // hard declaration
    let a : [i32; 5] = [5,4,3,2,1];

    // not only numbers
    let b = ["a", "b", "c", "b", "b"];

    //accessing array elements
    let first = a[0];
    let second = a[1];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}   