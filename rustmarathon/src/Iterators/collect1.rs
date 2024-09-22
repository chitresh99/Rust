//iterators in rust 

//iter() -> Helps go through each element in the collection 
//into_iter -> You can't use the variable used to store the elements again
//Consumes the collection and yields ownership of each element
//map -> Tranforms each element in the collection
//filter -> filter element based on certain condition 
//collect -> collects the results and transforms it into a new collection

fn main(){
    let numbers = vec![1,2,3,4,5];
    
    let doubled:Vec<_> = numbers.iter().map(|x| x*2).collect();
    println!("Doubled: {:?}", doubled);
}