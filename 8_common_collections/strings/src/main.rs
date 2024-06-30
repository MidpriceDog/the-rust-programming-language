fn main() {
    let mut s = String::from("hello");
    let s2 = " world";
    s.push_str(" foo bar");
    println!("{}", s);
    let s1 = s.clone();
    // The function signature of add is fn add(self, s: &str) -> String
    let _ = s1 + s2; // We can not make a reference to s anymore since it is moved.
   
    // The format macro takes a reference to every parameter.
    println!("{}", format!("{}, world!", s));

    // We cannot index into a string since it is a collection of bytes, represented
    // internally using a Vec<u8>
    let mut s3 = String::from("foobar");

    // Indexing into a string is often bad because it is not clear what 
    // the return type of the index operation is: a byte value, a char, or a 
    // grapheme cluster. So, instead we use string slices in Rust.
    let s3_slice = &s3[0..2];
    println!("{s3_slice}");

    //  Iteration
    for c in s3.chars() {
        println!("{c}");
    }

    // Replace range
    s3.replace_range(..1, "foo");
    println!("{s3}");


}
