fn main() {
    println!("{}", convert_c_to_f(20.0));
}

fn convert_c_to_f(temp_c: f32) -> f32 {
    return 9.0 * temp_c / 5.0 + 32.0;
}
