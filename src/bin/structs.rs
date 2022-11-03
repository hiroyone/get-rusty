#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height
    }

    fn can_hold(&self, others:&Rectangle) -> bool {
        return (self.width >= others.width) && (self.height >= others.height)
    }

    fn new_square(len:u32) -> Rectangle {
        Rectangle { width: len, height: len }
    }
}

fn main(){
    let rec1 =Rectangle {
        width:5,
        height:2
    };

    let rec2 =Rectangle {
        width:3,
        height:2
    };

    let square1 = Rectangle::new_square(3);

    println!("Rectangle: {:#?}", rec1);
    println!("The area of rectangle is {}", rec1.area());
    println!("rec1 can holds rec2: {}", rec1.can_hold(&rec2));
    println!("rec1 can holds square1: {}", rec1.can_hold(&square1));
}