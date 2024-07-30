fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is : {y}")
}

// {
//     let x = 3;
//     x + 1
// }

// in this case, evaluates to 4. That value gets bound to y as part of the let statement. 

//expressions don't have semicolons but they return a value