use chrono::{Local,Utc};

fn main() {
    let now = Utc::now();
    println!("This is the current data :{}",now);

    let local = Local::now();
    println!("This is the current data : {}",local); 
}

