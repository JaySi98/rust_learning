#![allow(unused_variables)]

fn main() {
    // Vector ------------------------------------------------------
    println!("\nVector example: ");
    vector_example();

    println!("\nVector with enum example: ");
    vector_with_enum_example();

    // String ------------------------------------------------------
    println!("\nString example: ");
    string_example();

    // Hash map ------------------------------------------------------
    println!("\nHash map example: ");
    hash_map_example();
}

fn vector_example() {
    // creating vector
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    // adding elements
    v.push(5);

    // accessing elements
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // saver option
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // iterating
    for i in &v {
        println!("{}", i);
    }

    // removing element
    v.pop();
}

fn vector_with_enum_example() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("vec = {:?}", i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn string_example() {
    // creating 
    let s = String::from("initial contents");
    let mut s  = "some text".to_string();
    println!("{}", s);

    // appending
    let s2 = "bar";
    s.push_str(s2);
    println!("{}", s);
    // single char
    s.push('l');
    println!("{}", s);

    // adding
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 no longer valid, needs to be &s2
    //
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // all values are still valid
    let s = format!("{}-{}-{}", s1, s2, s3);

    // string doesnt have indexing
    // let s1 = String::from("hello");
    // let h = s1[0];

    //iterating
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}

use std::collections::HashMap;

fn hash_map_example() {
    //
    let mut scores = HashMap::new();
    
    //
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{} = {}", team_name, score);

    //
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    
    //
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    //
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}