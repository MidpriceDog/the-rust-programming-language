struct User {
    active: bool,
    first_name: String,
    last_name: String,
    email: String,
    sign_in_count: u64,
}

fn build_user_longhand(fname: String, lname: String, email: String) -> User {
    User {
        active: true,
        first_name: fname,
        last_name: lname,
        email: email,
        sign_in_count: 1,
    }
}

// Here, we have use field init shorthand syntax where instead of having key-value
// pair mappings where the variable key is the same name as the value name, we
// simply provide the name once since the field and parameter names are the same.
fn build_user_shorthand(first_name: String, last_name: String, email: String) -> User {
    User {
        active: true,
        first_name,
        last_name,
        email,
        sign_in_count: 1,
    }
}

// Example of a tuple struct. Tuple structs are useful 1) when names for fields
// would be verbose or redundant or 2) the whole tuple deserves a name.

struct Color(u32, u32, u32);
struct Point(i32, i32, i32);

fn main() {
    // Create an instance of the User struct.
    let mut first_user = User {
        active: true,
        first_name: String::from("akira"),
        last_name: String::from("toriyama"),
        email: String::from("akira.toriyama@gmail.com"),
        sign_in_count: 1,
    };
    // We can use dot notation to access and modify fields of a mutable User
    // instance. Rust does not allow certain fields to be mutable while others
    // are not.
    first_user.email = String::from("akira.toryiama@outlook.com");
    let user_two = build_user_longhand(
        String::from("Yashimoto"),
        String::from("Nara"),
        String::from("yash.nara@gmail.com"),
    );
    // Example of struct update syntax. We supply the parameter to the field email
    // and use the remainder of the fields already filled in with parameter values
    // to initialize the rest of user_two.
    let user_two: User = User {
        // We can choose to specify whichever fields in whatever order we wish.
        email: String::from("akira.toriyama@msn.com"),
        ..first_user
    };

    let black: Color = Color(0, 0, 0);
    let unit_vec_i: Point = Point(1, 0, 0);
    let unit_vec_j: Point = Point(0, 1, 0);
    // We can access the fields in a tuple struct by index, similar to how
    // we can acccess fields by name in structs.
    println!("x component of unit vector i: {}", unit_vec_i.0);
    println!("y component of unit vector j: {}", unit_vec_j.1);
}
