enum Message {
    Quit,
    Move {x:u32,y:u32},
    Write(String),
    Changecolor(i32,i32,i32)
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::Changecolor(255, 0, 0);
    
    display_message(msg1);
    display_message(msg2);
    display_message(msg3);
    display_message(msg4);
}

fn display_message(msg:Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move{x,y} => println!("Move to coordinates: ({}, {})", x, y),
        Message::Write(text) => println!("Here is the text {}",text),
        Message::Changecolor(r, g, b) => println!("Change color to: RGB({}, {}, {})", r, g, b),
    }
}

