fn serve_order() {
    // code
}

// mod keyword is for defining modules
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_inccorect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {
        // code
    }
}

mod front_of_house;

pub use crate::front_of_house::hosting;
// this is also valid:
// self::front_of_house::hosting;

use std::fmt::Result;
// rename with 'as' keyword to avoid naming colisions
use std::io::Result as IoResult;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // 'use' keyword brings into scope
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("What");
    println!("I'd like {} toast please", meal.toast);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
