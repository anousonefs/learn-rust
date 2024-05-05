mod front_of_house;

#[allow(dead_code)]
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
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

use crate::front_of_house::hosting;
#[allow(dead_code)]
mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}
