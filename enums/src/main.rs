#![allow(unused_variables, unused_assignments, dead_code, unused_mut)]

fn main() {
    // Enums with parameters -----------------------------------------------------
    println!("\nEnums with parameters example 1: ");
    enums_example_1();

    //
    println!("\nEnums with parameters example 2: ");
    enums_example_2();

    // Option enum -----------------------------------------------------
    println!("\nOption enum example: ");
    options_enum_example();

    // Match -----------------------------------------------------
    println!("\nMatch example 1: ");
    match_example_1();

    println!("\nMatch example 2: ");
    println!("match_example_2(Some(5)): {}", match_example_2(Some(5)).unwrap());
    println!("match_example_2(Some(None)): {}", match_example_2(None).unwrap_or(0));
}

fn enums_example_1(){
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("IpAddr::V4: {:#?}", home);
    println!("IpAddr::V6: {:#?}", loopback);
}

fn enums_example_2(){
    let m = Message::Write(String::from("hello"));
    m.call();

    let m = Message::Quit;
    m.call();    
}

fn options_enum_example(){
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number.unwrap(): {}", some_number.unwrap());
    println!("absent_number.unwrap_or(69): {}", absent_number.unwrap_or(69));
}

fn match_example_1(){
    println!("value_in_cents(Coin::Penny): {}", value_in_cents(Coin::Penny));
    println!("value_in_cents(Coin::Nickel): {}", value_in_cents(Coin::Nickel));
    println!("value_in_cents(Coin::Dime): {}", value_in_cents(Coin::Dime));
    println!("value_in_cents(Coin::Quarter(UsState::Alabama)): {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}

fn match_example_2(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}