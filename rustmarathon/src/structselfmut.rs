//Mutuable self

struct Rectangle {
    width :u32,
    height :u32,
}

struct Square {
   side :u32,
}


impl Rectangle {
    fn resize(&mut self,new_width:u32,new_height:u32){
        self.width = new_width;
        self.height = new_height;
    }
}

impl Square {
    fn area1(&self) -> u32{
        self.side * self.side
    }
}

fn main() {
    let mut rect = Rectangle {width:10,height:10};
    rect.resize(15,25);
    println!("Updated Width: {}, Updated Height: {}", rect.width, rect.height);
    
    let square1 = Square {side : 10};
    println!("Area of a square is {}",square1.area1());
    
}