enum Shape {
    Rectangle(f64,f64),
    Circle(f64),
}

//Notes :- enum::the data

fn main() {
    let rect = Shape::Rectangle(1.0,2.0);
    calculate_area(rect);
    let circle = Shape::Circle(1.0);
    calculate_area(circle);
}

fn calculate_area(shape: Shape) -> f64{
    //pattern matching
    //we could have anything like a circle or a square
    match shape{
        Shape::Rectangle(a,b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}