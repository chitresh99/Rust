struct Person{
    name: String,
    age : u32,
    email : String,
}

fn main(){
    //creating a instance named person1
    let person1 = Person{
        name : String::from("Chitresh"),
        age : 30,
        email:String::from("chitresh@gmail.com"),
    };
    
    //accessing individual fields of a struct using a dot notation
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Email: {}", person1.email);
}