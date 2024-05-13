// Items in a parent module can’t use the private items inside child modules,
// but items in child modules can use the items in their ancestor modules.
// This design choice by Rust is so implementation details of child modules are
// hidden by default (child sees parent, parent does not see inside child).
// This can be overriden by explicility making an innder module pub.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {} 
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// This function is part of our library crate’s public API, so we mark it with
// the pub keyword.
pub fn eat_at_restaurant() {
    // Absolute path.
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path.
    front_of_house::hosting::add_to_waitlist();
}