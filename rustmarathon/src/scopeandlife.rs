fn main() {
    {
        let a = String::from("Temporary");
        // `a` is valid here
    } // `a` goes out of scope and is dropped here

    // println!("{}", a); // This would cause a compile-time error
}
