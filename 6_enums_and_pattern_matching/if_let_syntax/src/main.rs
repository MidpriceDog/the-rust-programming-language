// Verbose match expression. We can make this shorter by using an if-let .
fn prev_tx_id(tx_id: Option<u32>) {
    match tx_id {
        Some(x) => println!("The transaction id is {}", x),
        _ => (),
    }
}
// The if-let is syntactix sugar for a match expression that does one thing
// on a match, and nothing in any other case. It means less typing and indentation,
// but losing the exhasutive checking of a pattern match expression.
fn prev_tx_id_iflet_version(tx_id: Option<u32>) {
    if let Some(x) = tx_id {
        println!("The transaction id is {}", x);
    }
}

fn main() {
    println!("Hello, world!");
}
