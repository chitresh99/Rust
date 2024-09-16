fn main() {
    let maybe_value: Option<i32> = Some(10);
    let value = maybe_value.unwrap();
    println!("{}", value); // Outputs: 10
}