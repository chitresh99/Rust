enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("Hello");

    match some_number {
        Option::Some(value) => println!("We have a value: {}", value),
        Option::None => println!("No value"),
    }
}
