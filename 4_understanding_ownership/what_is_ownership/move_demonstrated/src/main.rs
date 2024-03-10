fn main() {
    // Create a string s1
    let s1 = String::from("hello");
    // In static memory, a pointer is created to data on the heap which stores
    // the index and value of string indices. Capacity and length are also stored.

    // When we create a variable s2, the static memory stores a copy of the capacity
    // and length and a pointer to the same heap data as is pointed to by s1 under the hood.
    let s2 = s1;

    // When a variable goes out of scope, Rust calls drop and frees up the heap memory.
    // Freeing memory twice (double free error) is a memory safety issue.
    // So what happens if we try to access s1 here?
    println!("{} world!", s1);
    // Rust no longer considers references to s1 valid. We say s1 was moved into s2,
    // rather than say Rust created a shallow copy, since the first variable was invalidated.
}
