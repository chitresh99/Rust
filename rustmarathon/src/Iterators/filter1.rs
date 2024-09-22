//iterators in rust 

//iter() -> Helps go through each element in the collection 
//into_iter -> You can't use the variable used to store the elements again
//Consumes the collection and yields ownership of each element
//map -> Tranforms each element in the collection
//filter -> filter element based on certain condition 

fn main(){
    let numbers = vec![1,2,3,4,5,6,7,8,9];
    
    let evens: Vec<_> =numbers.iter().filter(|x| *x %2 == 0).collect();
    println!("even numbers {:?}",evens);
    
}
