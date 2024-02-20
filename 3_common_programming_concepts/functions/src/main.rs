fn main() {
    println!("Hello, world!");
    another_function(4);
    print_labeled_measurement(12, 'b');
    expression_example();
    statement_example();
    let ans = function_with_return();
    println!("The function function_with_return() evaluates to {ans}");
    let s = subtract_two(ans);
    println!("Subtract two function: {ans} - 2 is {s}");
}

// You must declare the type of each parameter in function signatures.
// 1. This provides more helpful error messages.
// 2. This means the compiler almost never needs to use the annontations elsewhere to figure out what type you mean.
fn another_function(x : i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value : i32, unit_label : char) {
    println!("Measurement: {value}{unit_label}");
}

// Unlike C and C++, the result of an assignment does not return the value of the assignment.
// For example, x = y = 3 is equivalent to x = 3; y = 3; in these languages.
// In Rust, this is not the case. Statements like let x = 3 do not return a value.
fn statement_example() {
    let _x = 3; // Ok.
    // let y = (let z = 2); // Not ok. Code will not compile.
}

fn expression_example() {
    // The scope block with curly brackets is an expression.
    // Here, the value b + 2 is bound to a.
    // The value b + 2 is returned since no semicolon is at the end of the line.
    let a = {
        let b = 5;
        b + 2
    };
    println!("The value of a is {a}")
}

// Return value types in Rust are declared after an arrow after the name
fn function_with_return() -> i32 {
    // No semicolon because we want an expression returning 5, NOT a statement.
    5 
}

fn subtract_two(x : i32) -> i32 {
    return x-2;
}