#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // The functions defined in an impl block are called associated functions.
    // Associated functions are not neccessarily methods. Methods first parameter
    // is always self, which represents the instance of the struct.

    // Here we are borrowing self immutably.
    // If we needed to modify self, we would need to write: &mut self.
    // If we wanted to take ownership of self, we would write: self.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width > another_rect.width && self.height > another_rect.height
    }

    // In the function square, the keyword Self are aliases for the type that
    // appears after the impl keyword.
    // The function is namespaced by Rectangle, and can be called using Rectangle::square(3);
    fn square(x: u32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn main() {
    let my_rectangle = Rectangle {
        width: 30,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} pixels",
        my_rectangle.area()
    );

    let rect_1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect_2 = Rectangle {
        width: 10,
        height: 40,
    };

    print!("Can rect_1 hold rect_2? {}", rect_1.can_hold(&rect_2));
}
