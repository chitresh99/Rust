// Strings and slices in rust 

// String -> Dynamic . can grow in size
// string slice -> is immutable and doesn't own the data 


fn main(){
    let mut s = String::new();
    s.push_str("Hello, ");
    s.push('R');
    s.push_str("ust");
    
    println!("{}",s);
}