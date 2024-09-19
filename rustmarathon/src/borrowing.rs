fn main() {
    let s = String::from("hello");
    
    //immutable borrows
    let s_ref1 = &s;
    let s_ref2 = &s;

    println!("{}", &s_ref1);
    println!("{}", &s_ref2);

    let mut s_mut = s;
    s_mut.push_str("hello");

    println("{}",s_mut);
}