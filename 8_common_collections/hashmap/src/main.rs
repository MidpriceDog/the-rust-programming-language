use std::{
    collections::HashMap,
    io::{self, Read},
};

/* Given a list of integers, use a vector and return the median (when sorted,
* the value in the middle position) and mode (the value that occurs most often;
* a hash map will be helpful here) of the list.
*/
fn median(list_of_values: Vec<i32>) -> f32 {
    if list_of_values.len() % 2 == 0 {
        let middle_right = list_of_values[list_of_values.len() / 2];
        let middle_left = list_of_values[list_of_values.len() / 2 - 1];
        (middle_left + middle_right) as f32 / 2.0
    } else {
        list_of_values[list_of_values.len() / 2] as f32
    }
}

/*Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words  that start with a vowel have “hay” added to the end instead (“apple” becomes“apple-hay”). Keep in mind the details about UTF-8 encoding!
*/
fn pig_latin(input_str: &mut String) -> String {
    let vowels: Vec<&str> = vec!["a", "e", "i", "o", "u"];
    match input_str.chars().next() {
        Some(x) => {
            if vowels.contains(&x.to_string().as_str()) {
                input_str.push_str("hay");
                input_str.to_string()
            } else {
                input_str.push(x);
                input_str.push_str("ay");
                (&input_str[1..]).to_string()
            }
        }
        None => input_str.to_string(),
    }
}

/*
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
 */
fn text_interface() -> String {
    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let (first, _) = input.split_at(input.find(' ').unwrap());
    first.to_string()
}

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("dog"), 1000);
    map.insert(String::from("cat"), 200);
    // The call to copied gets an Option<i32> rather than an Option<&i32>
    let price: i32 = map.get("dog").copied().unwrap_or(0);
    println!("{price}");

    for (key, value) in &map {
        println!("{key} : {value}");
    }

    // Ownership with HashMap example
    let mut name_ticker = HashMap::new();
    let company1 = String::from("Apple");
    let company2 = String::from("Microsoft");
    name_ticker.insert(company1, String::from("AAPL"));
    name_ticker.insert(company2, String::from("MSFT"));
    // The map takes ownership of company1 and company2 and they cannot be used
    // let new_string = company1 + &String::from(" hello."); this causes a panic
    let v: Vec<i32> = vec![1, 23, 34, 45, 53];
    let v2: Vec<i32> = vec![1, 2];
    let mut x = String::from("apple");
    println!("{}", median(v));
    println!("{}", median(v2));
    println!("{}", pig_latin(&mut x));
    println!("{}", text_interface());
}
