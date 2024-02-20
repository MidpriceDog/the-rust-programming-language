use std::num::Saturating;
use std::io;

fn index_out_of_bounds_ex(){
    // Initialize array a to have 10 elements each with value 1
    let a = [1; 10];
    println!("Please enter a number: ");
    // Create a new string using the String struct 
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index) // read line into variable into by making a mutable reference
        .expect("Failed to read line."); // Handle the Result that is returned by the read_line method

    let index : usize = index
        .trim()
        .parse()
        .expect("Input provided is not a number.");
    // If index is out of bounds, a runtime error will occur and thread 'main' will panic
    // This is an example of Rust's memory safety features at work
    let element = a[index];
    println!("Woohoo! The number you entered is a valid index!");

}

fn main() {
    let mut x = 1;
    println!("The value of x is {x}.");
    x = 3;
    println!("The value of x is now {x}.");
    let spaces = "  ";
    let spaces = spaces.len();
    let a : Saturating<u8> = Saturating(250);
    let b : Saturating<u8> = Saturating(23);
    let sum = a + b;
    println!("Saturating Sum: {a} + {b} = {sum}");

    let x = 2.0; // default float is f64 since precision is higher and speed is roughly the same as f32
    let y : f32= 3.5 ;

    let boolean : bool = true; // one byte
    let heart_cat : char = '😻'; // four bytes and represents Unicode Scalar Value
    println!("{}", heart_cat);

    // The tuple type is a compound type
    // Once declared, the type has a fixed length
    let tup : (i32, u8, f64) = (500, 1, 3.423);
    // Use pattern matching to destructure a tuple value into separate variables
    let (x,y,z) = tup;
    println!("The value of the 1st element in the tuple is {x}");
    println!("The value of the 2nd element in the tuple is {y}");
    println!("The value of the 3rd element in the tuple is {z}");
    // We can access the values in the tuple using dot notation too
    let first_element = tup.0;
    println!("{}", first_element == x);

    // Write an array's type using square brackets with type of elem
    let arr : [i32; 5] = [1,40,0,12,0];
    // We can initialize an array with a value and then the length too
    // Here, the array will be initialized with 5 elements each set to the value 3 
    let arr = [3; 5];

    index_out_of_bounds_ex();

}
