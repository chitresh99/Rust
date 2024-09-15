fn main() {
    // let x:i32 = 1;
    println!("{}",fib(4));
}
 
 // 0 1 1 2 3 
 
//i32 is the return type
fn fib(num:u32) -> u32{
    let mut first = 0;
    let mut second = 1;

    //First Base case
    if num == 0 {
        return first;
    }
    
    //second base case
    if num == 1 {
        return second;
    }
    
    //right here we move one by one , one number at a time , we first store the second number and increment it we move the second number basicallly
    // and the first number becomes the second number in which we are also moving it 
    //we can use just '_' or _i
    for _ in 0..num - 1{
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}