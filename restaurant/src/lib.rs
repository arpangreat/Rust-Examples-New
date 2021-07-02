mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}

        pub mod back_of_house {
            pub fn fix_incorrect_order() {
                cook_order();
                super::serve_order();
            }

            pub fn cook_order() {}

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
        pub fn take_payment() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    // Absoulute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();

    // Order for a summer Breakfast
    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    // Change the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast plz..", meal.toast);
}
