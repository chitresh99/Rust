// s.chars -> returns you the characters in the string
//.count -> returns the total number of characters in the string 
// This is an implicit return we don't specify we are returning something but it's still returning something

fn get_string_length(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    //String::from -> This is one way of defining a string there are many
    let my_string = String::from("Hello world"); //space also counts as one character
    let length = get_string_length(&my_string);
    println!("The number of characters in the string is : {}",length);
}