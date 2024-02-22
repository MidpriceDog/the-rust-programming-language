use std::io;
use std::time::Instant;

fn main() {
    looping_array_with_while();
    looping_array_with_for();
}

fn age_conditional() {
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read age.");
    let age = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    // The if expression does not need parentheses like in some other languages
    if age > 21 {
        println!("Congrats! Here is a drink.");
    } else {
        println!("Sorry, can't serve you.");
    }
}


// Rust does not convert non-boolean values to booleans automatically like some other languages.
fn conditional() -> bool {
    // if 1 { // This will not compile.
    if true { // This will compile.
        return true;
    } else {
        return false;
    }
}

fn else_if_branches(x : i32) {
    if x < 18 {
        println!("Can't vote!");
    }
    else if x > 18 && x < 21 {
        println!("Can vote but not drink!");
    }
    else {
        println!("Voting and drinking age!");
    }
}

fn returning_val_from_loop(x : i32) {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == x {
            break x * 2;
        }
    };
    println!("Result : {result}");
}

fn labeled_loops() {
    let mut outer_counter = 0;
    let mut inner_counter = 10;
    // We can name loops using the syntax 'name
    'outer_loop_label : loop {
        println!("The outer loop is at {outer_counter}");
        // Increment outer counter 
        outer_counter += 1;
        'inner_loop : loop {
            if inner_counter == 1 {
                break 'outer_loop_label;
            }
            inner_counter -= 1;
        }
    }
    println!("The outer counter finished at {outer_counter}.");
    println!("The inner counter finished at {inner_counter}");
}

fn while_loop() {
    let mut lift_off = 10;
    while lift_off > 0 {
        println!("{lift_off}...");
        lift_off -= 1;
    }
    println!("LAUNCH");
}

fn looping_array_with_while() {
   
    let start = Instant::now();

    let mut index = 0;
    let array = [3; 1000*1000];
    let mut result = 0;
    while index < array.len() {
        result += array[index];
        index += 1;
    }
    println!("\n");
    println!("Result: {}, Index: {}", result, index);
    let duration = start.elapsed();
    println!("while execution time: {:?}", duration);
    println!("\n");
}

fn looping_array_with_for() {
    let start = Instant::now();
    let mut index = 0;
  
    let array = [3; 1000*1000];
    let mut result = 0;
    for element in array {
        result += element;
        index += 1;
    }
    println!("Result: {}, Index: {}", result, index);
    let duration = start.elapsed();
    println!("for loop execution time: {:?}", duration);
    println!("\n");
}