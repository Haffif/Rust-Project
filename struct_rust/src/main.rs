#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
     let rect = Rectangle {
        width: 30,
        height: 50,
     };

     println!("{:?}", rect);
     println!("The are of rectangle is {}", area(&rect));
}

fn area(size: &Rectangle) -> u32 {
    size.width * size.height
}
