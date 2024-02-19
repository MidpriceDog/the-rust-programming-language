use std::num::Saturating;

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

}
