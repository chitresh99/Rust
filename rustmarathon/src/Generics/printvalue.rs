// A generic function that accepts any type `T`
// But `T` must implement the `Display` trait so it can be printed.
fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

fn main() {
    print_value(10);           // Works because `i32` implements Display
    print_value(3.14);         // Works because `f64` implements Display
    print_value("Hello Rust"); // Works because `&str` implements Display
}
