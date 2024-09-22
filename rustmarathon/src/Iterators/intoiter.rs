//iterators in rust 

//iter() -> Helps go through each element in the collection 
//into_iter -> You can't use the variable used to store the elements again
//Consumes the collection and yields ownership of each element

fn main () {
    let numbers = vec![1,2,3,4,5];
    let into_iter = numbers.into_iter();

    for num in into_iter{
        println!("owned: {}",num);
    }
    
    // `numbers` is no longer accessible here because it's consumed
    // println!("{:?}", numbers); // This would cause an error
}

