fn main() {
    // let mut s = String::from("hello");
    // s.push_str(" world");
    // println!("{}", s);

    let x = 5;
    let y = x; // copy
    println!("x = {}. y = {}", x,y);

    let s1 = String::from("hello");
    let s2 = s1; // moved
    println!("{}", s2);

    // Apabila mau muncul string1 maka gunakan clone
    let str1 = String::from("hello");
    let str2 = str1.clone(); // moved
    println!("str1 = {} str2 = {}", str1 ,str2);

    let string1 = gives_ownership();
    println!("{}", string1);

    let string2 = String::from("hello");
    //println!("{}", string2);

    let string3 = takes_and_gives_back(string2);
    println!("{}", string3);

    let tup1 = String::from("tuple");
    let (tup2, len) = calculate_length(tup1);
    println!("The length of {} is {}", tup2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    let some_str = String::from("world");
    some_str
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
