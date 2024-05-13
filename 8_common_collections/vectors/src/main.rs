fn main() {
    println!("Hello, world!");
    // Explicitly specify the type of the vector since it is empty.
    let v : Vec<i32> = Vec::new();
    // Here the Vec<i32> type is inferred by Rust since we initialize vector with values.
    let v2 = vec![1,2,3];
    // No type annotation needed for v3 either since we push values to it.
    let mut v3 = Vec::new();
    v3.push(32);

    // Values can be accessed using indexing or the get() method.

    // Using the index, which will cause Rust to panic if index is out of bounds.
    let third : &i32 = &v2[2];
    println!("The third element of v2 is {}", third);

    // Using the get method, which returns an Option<T>
    let third_using_get : Option<&i32> = v2.get(2);
    match third_using_get {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    // the borrow checker ensures that references to contents of a vector 
    // are made only when the vector itself is valid.
    {
        let v4 = vec![1,2,3,4,5];
        let first = &v4[0];
    }// v4 goes out of scope here and is freed.

}
