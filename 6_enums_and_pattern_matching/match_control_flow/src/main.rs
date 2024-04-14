enum Coin {
    Nickel,
    Penny,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Nickel => 5,
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn lucky_penny(coin: Coin) -> u8 {
    // Note: Arm patterns must cover all possibilities i.e. matches in Rust are
    // exhaustive. Not including all variants in the match will cause a compile
    // time error.
    match coin {
        Coin::Nickel => 5,
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin_i_found = Coin::Penny;
    value_in_cents(coin_i_found);
    let something_or_nothing = Some(34);
    let incr = plus_one(something_or_nothing);
    println!("{:?}", incr);
}
