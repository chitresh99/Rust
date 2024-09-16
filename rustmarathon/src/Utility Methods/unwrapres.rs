fn main (){
    let result: Result<i32, &str> = Err("Something went wrong");
   let value = result.unwrap(); // Panics: "called `Result::unwrap()` on an `Err` value"
   println!("The value is {} ",value);
}