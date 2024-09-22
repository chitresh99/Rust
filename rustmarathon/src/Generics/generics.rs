struct Point<T>{
    x: T,
    y:T,
}

fn main(){
    let int_point = Point { x: 5, y: 10 };
    println!("Point: ({}, {})", int_point.x, int_point.y);

    // Point of floating-point numbers
    let float_point = Point { x: 1.2, y: 3.4 };
    println!("Point: ({}, {})", float_point.x, float_point.y);
}