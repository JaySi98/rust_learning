#![allow(dead_code, unused_imports)]

mod front_of_house;

//Paths ----------------------------------
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // works thanks to use... above
    hosting::add_to_waitlist();

    // doesnt work
    // add_to_waitlist();
}

//Super keyword ----------------------------------
fn deliver_order() {}

mod example {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

//Public structs and enums ----------------------------------
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // this is public
        seasonal_fruit: String, // this is private
    }

    // struct needs constructor if it has private fields 
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // everything is public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn get_toast() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

//Bringing into module ----------------------------------
use std::fmt::Result;
use std::io::Result as IoResult; // renamed so it doesnt collide with other Result
//
use std::io::{self, Write}; // brings std:io and std::io::Write
//
use std::collections::*; // brings everything public 
