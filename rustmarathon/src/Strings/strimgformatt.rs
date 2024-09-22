// string formatting

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    
    let s3 = format!("{} {}", s1, s2);
    println!("{}", s3); // Output: Hello Rust
}