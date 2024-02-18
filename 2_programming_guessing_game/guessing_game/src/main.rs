use std::io;
use rand::Rng; // Rng is a trait. It defines methods that random number generators implement.
use std::cmp::Ordering; // Ordering is an enum with variants Less, Greater, and Equal

fn main() {
    println!("Guess the number game! \n");
    // The gen_range method is brought into scope with the use rand::Rng; statement
    // The gen_range method is inclusive on lower and upper bounds
    let secret_num = rand::thread_rng().gen_range(1..=100);
    
    loop { 
        println!("Input your guess: ");
        // Variable guess must be mutable so the method read_line can append user input to the string
        let mut guess = String::new(); 
        // Use & to obtain a reference to variable instead of creating another copy in memory
        // References are immutable by default so we write &mut guess rather than &guess 
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        // The parse method returns a Result type, which is an enum with variants Ok and Err.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>  continue,
        };

        println!("Your guess was: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Guess is too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Guess is too big"),
        }
    }
}
