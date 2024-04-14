use crate::garden::vegtables::Asparagus;

pub mod garden;

fn main() {
    
    let plant = Asparagus {
        weight: 10,
        is_rotten: false,
    };
    println!("Hello, world!");
}
