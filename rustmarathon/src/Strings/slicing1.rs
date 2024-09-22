// Strings and slices in rust 

// String -> Dynamic . can grow in size
// string slice -> is immutable and doesn't own the data 
//modfying -> You can mutate (push, push_str, etc.) a String because it is growable.


//slice


fn main(){
    let s = String::from("hello,rust");
    
    //slicing part of a string
    
    let hello = &s[0..5];
    let rust = &s[6..10];
    
    
    
     println!("First slice: {}, Second slice: {}", hello, rust);
}