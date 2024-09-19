fn main(){
    let x = String::from("hello");
    let y = x; // the x is assigned to y
    //the ownership of the string is transferred from 'x' to 'y'

    println!("{}",y); //this works because 'y' owns the string
}