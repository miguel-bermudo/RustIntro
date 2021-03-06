
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn instantiate(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

}

pub use crate::front_of_house::hosting;

mod customer{
    use crate::back_of_house;

    pub fn eat_at_restaurant(){
        let mut map = HashMap::new();
        map.insert(1, 2);

        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();
        // Relative path
        super::front_of_house::hosting::add_to_waitlist();
        // Use declaration
        super::hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::instantiate("Rye");
        
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");


        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}