fn borrow_example() {
    let mut s = String::from("Hello, World.");
    change_str(&mut s);
}

fn change_str(s: &mut String) {
    s.push_str(" Nice weather isn't it?");
}

fn two_mutable_references_at_different_times() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here. No issue with making a new reference.
    let r2 = &mut s;
    print!("{}", r2);
}

fn main() {
    borrow_example();
    two_mutable_references_at_different_times()
}
