use std::io;

fn main() {
    println!("Guess the number!");
    println!("Input your guess: ");
    // Variable guess must be mutable so the method read_line can append user input to the string
    let mut guess = String::new(); 
    // Use & to obtain a reference to variable instead of creating another copy in memory
    // References are immutable by default so we write &mut guess rather than &guess 
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    let x = io::stdin().read_line(&mut guess).expect("error");
    println!("{x}");
    println!("Your guess was: {guess}");
}
