fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count : {}", count);

        if count == 5{
            break;
        }
    }

    println!("Loop ended");
}