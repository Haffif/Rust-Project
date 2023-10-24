#[derive(Debug)]
#[warn(dead_code)]
// enum ipAddRKind{
//     V4,
//     V6,
// }

// struct ipAddr{
//     kind: ipAddRKind,
//     address: String,
// }

enum IpAddrNew {
    V4(String),
    V6(String),
}

enum Message {
    Quit, 
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {

    // let home = ipAddr {
    //     kind: ipAddRKind::V4,
    //     address: String::from("127.0.0.1"), 
    // };
    let home = IpAddrNew::V4(String::from("127.0.0.1"));
    let home_six = IpAddrNew::V6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    println!("{:?}, {:?}", home, home_six);

    let msg = Message::Write(String::from("hello"));
    msg.call();

    let x = 5;
    let y: Option<i32> = Some(5);

    let sum = x + y.unwrap();

    println!("{}", sum);


}
