// Strings and slices in rust 

// String -> Dynamic . can grow in size
// string slice -> is immutable and doesn't own the data 
//modfying -> You can mutate (push, push_str, etc.) a String because it is growable.

fn main() {
    
    let mut s = String::from("Hello");
    s.push_str("world"); //push_str is for the whole word while push is just for a letter
    s.push("!");
    
    println!("{}",s);
}