use std::fs::File;

fn main() {
    let greeting_result = File::open("hello.txt");
    let greeting = match greeting_result {
        Ok(file) => file,
        Err(e) => panic!("Something went wrong reading the file: {}", e),
    };
}
