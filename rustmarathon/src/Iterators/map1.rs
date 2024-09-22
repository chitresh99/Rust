//iterators in rust 

//iter() -> Helps go through each element in the collection 
//into_iter -> You can't use the variable used to store the elements again
//Consumes the collection and yields ownership of each element
//map -> Tranforms each element in the collection


fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Using map() to square each element
    let squares: Vec<_> = numbers.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares); // Prints [1, 4, 9, 16, 25]
}


