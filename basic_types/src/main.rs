fn main() {
    // Mutability ------------------------------------------------------
    println!("\nMutability: ");
    mutability_example();

    // Shadowing --------------------------------------------------------
    println!("\nShadowing: ");
    shadowing_example();

    // Scalar types -----------------------------------------------------
    println!("\nScalar types: ");
    scalar_types_example();

    // Compound types -----------------------------------------------------
    println!("\nCompound types: ");
    compound_types_example();    

    // Str type -----------------------------------------------------
    println!("\nStr type: ");
    str_example();
}

#[allow(unused_variables, unused_assignments, dead_code)]
fn mutability_example(){
    // by default variables are immutable(cannot be changed)
    // this is not fine
    let immutable = 5;    
    // immutable = 6;

    // this is fine
    let mut mutable = 5;
    mutable = 6;

    // upper case conveniton, needs type
    const CONST_VAL:u8 = 5;
}

fn shadowing_example(){
    let x = "text";

    {
        let x = x.len();
        println!("x in the inner scope is: {x}");
    }

    println!("x outside the scope: {x}");
}

#[allow(unused_variables, unused_assignments, dead_code)]
fn scalar_types_example(){
    // integers 
    // i8 i16 i32 i64 i128 isize 
    // u8 u16 u32 u64 u128 usize
    let decimal: u32 = 98_222;
    let hex: u8 = 0xff;
    let octal: u8 = 0o77;
    let binary: u8 = 0b1111_0000;
    let byte: u8 = b'A';
    println!("decimal:  {}", decimal);
    println!("hex:      {}", hex);
    println!("octal:    {}", octal);
    println!("binary:   {}", binary);
    println!("byte:     {}", byte);

    // floats:
    // f32 f64
    let f1: f32 = 5.65;
    let f2: f64 = 5.65;
    println!("f32:      {}", f1);
    println!("f64:      {}", f2);

    // bool:
    let truth: bool = false;
    println!("f64:      {}", f2);

    // char:
    let c: char = 'z';
    println!("char:     {}", c);
    let c: char = 'ℤ'; // with explicit type annotation
    println!("char:     {}", c);
    let c: char = '😻';
    println!("char:     {}", c);
}

#[allow(unused_variables, unused_assignments, dead_code)]
fn compound_types_example(){
    // tuple
    let tuple: (char, u8, f32) = ('h', 8, 8.0);
    // println!("tuple:      {}", tuple); // doesnt work
    println!("tuple.0:  {}", tuple.0);
    println!("tuple.1:  {}", tuple.1);
    println!("tuple.2:  {}", tuple.2); 

    // array
    let arr: [i8; 5] = [1,2,3,4,5];
    let arr: [i8; 5] = [3;5]; // [3,3,3,3,3]
    // println!("array:        {}", arr);  // doesnt work
    // println!("array[5]:     {}", arr[5]);  // doesnt work
    // println!("array[-1]:    {}", arr[-1]);  // doesnt work
    println!("array[0]: {}", arr[0]);  // doesnt work
}


fn str_example(){

    let s = "ee";

    // switch to String for it to work
    // s = s + "aa";

    print!("{}\n", s);
}