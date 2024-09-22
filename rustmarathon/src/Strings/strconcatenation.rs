// Strings and slices in rust 

// String -> Dynamic . can grow in size
// string slice -> is immutable and doesn't own the data 
//modfying -> You can mutate (push, push_str, etc.) a String because it is growable.


//slice


//concatenation -> use & 

fn main(){
    
    let s1 = String::from("Hello,");
    let s2 = String::from("Rust");
    
    let s3 = s1 + &s2;
    println!("{}", s3);
    
}

//  let s3 = &s1 + &s2;
//   |              --- ^ --- &String
//   |              |   |
//   |              |   `+` cannot be used to concatenate two `&str` strings
//   |              &String
