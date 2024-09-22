use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();
    
    users.insert(String::from("harkirat"), 21);
    users.insert(String::from("Chitresh"), 21);
    
    let first_user_age = users.get("harkirat");
    
    match first_user_age {
        Some(age) => println!("age is {}", age),
        None => println!("User not found in the db"),
    }
}