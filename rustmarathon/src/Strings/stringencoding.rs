fn main() {
    let s = String::from("こんにちは"); // Japanese characters
    let slice = &s[0..1]; // Error: This is not valid because each character is more than 1 byte in UTF-8
}
