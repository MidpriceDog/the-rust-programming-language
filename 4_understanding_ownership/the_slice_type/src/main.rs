fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn other_slices() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[1, 3]);
}

fn main() {
    let mut my_string: String = String::from("Hello there.");
    let word = first_word(&my_string);
    println!("{}", word);
    my_string.clear();
    // word_index has value 5 but no more string we can meaningfully use the
    // value 5 with.
    // We can use String slices provided by Rust to keep the variables synced.
    // A string slice is a reference to part of a String.
    let s = String::from("Hello world!");
    // The type &str signifies a string slice.
    let he = &s[..2];
    let after_hello = &s[5..];
    println!("{}", he);
    println!("{}", after_hello);

    other_slices();
}
