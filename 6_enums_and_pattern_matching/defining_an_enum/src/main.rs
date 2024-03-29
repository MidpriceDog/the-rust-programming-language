// enums give us a way of saying a value if one of multiple values (variants).
enum IpAddrKind {
    // Both V4 and V6 variants have associated String values.
    V4(String),
    V6(u32, u32, u32, u32),
}

// The Option enum exists to represent the idea that a variable can be something
// or nothing at all. Rust does not have the null feature because it leads to many
// errors: if you try to use a null value as a non-null value, an error occurs.

enum Option<T> {
    Some(T),
    None,
}

// Q: So why is Option<T> better having null?
// A: Because the compiler will not allow us to use an Option<T> value as if it
// it were definietly a valid value (i.e. the code will not compile).

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Message::Quit {} => write!(f, "Quit"),
            Message::Move { x, y } => write!(f, "({}, {})", x, y),
            Message::Write(x) => write!(f, "{}", x),
            Message::ChangeColor(r, g, b) => write!(f, "({},{},{})", r, g, b),
        }
    }
}

// Alternatively, we could declared the enum variants each as their own struct,
// but then we would not be able to create a function that takes any of these
// kinds of messages as easily. See below:
struct Quit; // unit struct
struct Move {
    x: i32,
    y: i32,
}
struct Write(String); // tuple struct (no named fields)
struct ChangeColor(i32, i32, i32); // another tuple struct

//

fn main() {
    println!("Hello, world!");
    let internet_address_v4 = IpAddrKind::V4(String::from("127.0.0.1"));
    let message: Message = Message::Quit;
    let second_message: Message = Message::Move { x: 10, y: 23 };
    println!("{}", second_message);
    println!("{}", message);
}
