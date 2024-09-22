struct User<'a>{
    name: &'a str
}

fn main(){
    let first_name = String::from("Chitresh");
    let user = User{name: &first_name};
    
    println!("The name of the user is {}",user.name);
}