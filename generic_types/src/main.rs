#![allow(dead_code)]

use std::fmt::Display;
use aggregator::{Summary, Tweet, NewsArticle};

fn main() {
    // Generic parameter ------------------------------------------------------
    println!("\nGeneric parameter example: ");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['2','8', 'a', 'z'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Generic struct ------------------------------------------------------
    println!("\nGeneric struct example: ");
    let integer = Point {x: 5,   y: 10};
    let float   = Point {x: 1.0, y: 4.0};
    println!("{:?}", integer);
    println!("{:?}", float);

    // Traits ------------------------------------------------------
    println!("\nTraits example: ");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    // Traits as parameters ------------------------------------------------------
    println!("\nTraits as parameters example: ");


    // Lifetime annotations  ------------------------------------------------------
    println!("\nLifetime annotations example: ");
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string of {} and {} is {}", string1, string2, result);
}

//
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
// works on all structs that have Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// both paramters are the same type and have Summary trait
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// parameters are different type but have Summary trait
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// this     
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// is equal to this
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

//
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// this doesnt work
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }