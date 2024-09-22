//iterators in rust 

//iter() -> Helps go through each element in the collection 

fn main(){
    let numbers = vec![1,2,3,4,5];
    
    let iter = numbers.iter();
    for num in iter {
        println!("Borrowed : {}", num);
    }
    
    println!("Original vector : {:?}",numbers);
}