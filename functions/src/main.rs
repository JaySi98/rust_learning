fn main() {
    // Parameters -----------------------------------------------------
    println!("\nParameters example: ");
    parameters_example(8, 'z');

    // Expression -----------------------------------------------------
    expression_example();

    // Return example -------------------------------------------------
    println!("return_example(8) -> {}", return_example(8));
    println!("return_example(4) -> {}", return_example(4));
}

// types must be specified
fn parameters_example(number: u8, chr:char){
    println!("parameters_example called with: {}, {}", number, chr);
}

// with ; - statement
// without ; - expression
fn expression_example(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn return_example(x: u8)->u8 {
    if x > 5 {
        return x - 4;
    }

    x + 1
}