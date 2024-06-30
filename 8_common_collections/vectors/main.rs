use std::collections::HashMap;

fn main() {
    let map = HashMap::new();
    map.insert(String::from("dog"), 1000);
    map.insert(String::from("cat"),  200);

    let price : i32 = map.get("dog").copied().unwrap_or(0);
    println!("{price}");
}
