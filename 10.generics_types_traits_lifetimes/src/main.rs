mod generics;
use generics::{Point, MixedPoint};

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
}

fn largest_abstract(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_of_two_lists_reference(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_abstract(&number_list);
    println!("The largest number is {}, and the list is unmutated {:?}", result, number_list);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_abstract(&number_list);
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