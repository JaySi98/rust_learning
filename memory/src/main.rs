fn main() {
    // Borrowing -----------------------------------------------------
    println!("\nBorrowing example 1: ");
    borrowing_example_1();

    println!("\nBorrowing example 2: ");
    let text = String::from("hello");
    borrowing_example_2(text);
    // wont work, text already destroyed 
    // print!("{}\n", text); 

    // Reference -----------------------------------------------------
    // reference has to have the same mutability as variable 
    println!("\nReference example 1: ");
    let text = String::from("hello");
    let lenght = reference_example1(&text);
    println!("{} is {} letters long", text, lenght);

    println!("\nReference example 2: ");
    reference_example2();

    // Slice -----------------------------------------------------
    println!("\nSlice example 1: ");
    let text = String::from("hello world");
    let word = slice_example_1(&text);
    println!("First word in \'{}\' is {}", text, word);

}

fn borrowing_example_1(){    
    let s1 = String::from("hello");
    let s2 = s1;

    // doesnt work, value moved to s2
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    // now it works, created deep copy 
    let s1 = s2.clone();
    println!("s1: {}", s1);

}

fn borrowing_example_2(text: String){

    println!("{}", text);
} // text is destroyed here

fn reference_example1(text: &String)->usize {
    // wont work, text is immutable
    // text.push_str("world");

    text.len()
}

fn reference_example2(){
    let mut s = String::from("mutable string");

    let r1 = &mut s;

    // only one mut reference at a time 
    // let r2 = &mut s;

    // immutable references cant exist at the same time with mutable references
    // let r2 = &s; 

    println!("{}", r1);
}

fn slice_example_1(text: &str)->&str {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[0..i];
        }
    }

    &text[..]
}