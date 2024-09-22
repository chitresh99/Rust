fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}",is_even(vec));
}

fn is_even(vec:Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new(); //intializing it to an empty vector 
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}

