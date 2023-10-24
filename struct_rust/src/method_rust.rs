#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
            Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
     };
     let rect1 = Rectangle {
        width: 50,
        height: 40,
     };
     let rect2 = Rectangle {
        width: 40,
        height: 30,
     };
     let rect3 = Rectangle {
        width: 70,
        height: 60,
     };

     let sq = Rectangle::square(10);

     println!("The are of rectangle is {}", rect.area());
     println!("The width is nonzero, it is {}", rect.width());

     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

     println!("The value: {:?}", sq);
}

// fn area(size: &Rectangle) -> u32 {
//     size.width * size.height
// }
