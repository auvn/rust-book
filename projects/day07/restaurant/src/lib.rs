mod front {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}

        pub struct Breakfast {
            pub toast: String,
            fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    fruit: String::from("summer fruit"),
                }
            }
        }
    }

    mod hidden {
        fn cook_order() {}

        fn fix_order() {
            cook_order();
            super::serving::serve_order();
        }
    }
}

use std::collections::HashMap as MyMap;

use crate::front::hosting;

fn visit_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    let mut meal = front::serving::Breakfast::summer("Rye");
    meal.toast = String::from("Super Rye");

    println!("{} toast please!", meal.toast);
}

fn main() {
    let mut map = MyMap::new();
    map.insert(1, 1);
}
