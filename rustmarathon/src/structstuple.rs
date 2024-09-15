struct Color(i32,i32,i32);

fn main() {
    let black = Color(0, 0, 0);
    
    println!("Red: {}", black.0);
    println!("Green: {}", black.1);
    println!("Blue: {}", black.2);
}