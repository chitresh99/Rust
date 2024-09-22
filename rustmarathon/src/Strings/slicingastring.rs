fn main() {
    let s = String::from("hello");

    let slice = &s[0..2]; // Get a slice of the first two characters
    println!("Slice: {}", slice); // Output: he
}
