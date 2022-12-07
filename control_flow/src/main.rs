fn main() {

    // If -----------------------------------------------------
    println!("\nIf-else example: ");
    if_example();

    // Loop -----------------------------------------------------
    println!("\nLoop example: ");
    loop_example();

    // For -----------------------------------------------------
    println!("\nFor example: ");
    for_example();
}

fn if_example(){
    let number = 3;

    // statement must always be bool
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six"}; // doesnt work
    println!("The value of number is: {number}");
}

#[allow(unused_labels)]
fn loop_example(){

    // this is valid
    // loop {
    //     println!("again!");
    // }

    //

    let mut counter = 0;

    // 'counting_up: - name of the loop
    let result = 'counting_up: loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }

        // break 'counting_up;
    };

    println!("The result is {result}");
}

fn for_example(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //
    
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
