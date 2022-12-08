#![allow(unused_variables, unused_assignments, dead_code, unused_mut)]

fn main() {
    // Creating User instance ----------------------------------------------
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:#?}", user1);

    // user 1 no longer valid
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{:#?}", user2);

    //
    let color = Color(0, 123, 255);
    println!("color: {}, {}, {}", color.0, color.1, color.2);

    //
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let square = Rectangle::square(3);
    println!("{:#?}", square);
}

// tuple structure
struct Color(i32, i32, i32);

// unit structure, no parameters
struct AlwaysEqual;

// normal structure, printable
#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
