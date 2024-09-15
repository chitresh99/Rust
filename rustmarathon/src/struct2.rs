//mutuable structs

struct Person {
    name : String,
    age : u32,
    email : String,
}

fn main () {
    let mut person1 = Person{
        name : String::from("Chitresh"),
        age : 26,
        email : String::from("chitresh@gmail.com"),
    };
    
    //modifying the age field
    person1.age = 30;
    
    println!("Name: {}",person1.name);
    println!("age: {}",person1.age);
    println!("Email: {}",person1.email);
}