mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::{hosting, serving};
pub use crate::back_of_house::cooking;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    serving::manage_complaint();
    serving::serve_order();
}