fn main() {
    // // let mut s = String::from("hello");
    // // s.push_str(" world");
    // // println!("{}", s);

    // let x = 5;
    // let y = x; // copy
    // println!("x = {}. y = {}", x,y);

    // let s1 = String::from("hello");
    // let s2 = s1; // moved
    // println!("{}", s2);

    // // Apabila mau muncul string1 maka gunakan clone
    // let str1 = String::from("hello");
    // let str2 = str1.clone(); // moved
    // println!("str1 = {} str2 = {}", str1 ,str2);

    // let string1 = gives_ownership();
    // println!("{}", string1);

    // let string2 = String::from("hello");
    // //println!("{}", string2);

    // let string3 = takes_and_gives_back(string2);
    // println!("{}", string3);

    let tup1 = String::from("tuple");
    let len = calculate_length(&tup1);
    println!("The length of {} is {}", tup1, len);

    let mut str_str = String::from("Nice to Meet You");
    change(&mut str_str);

    // contoh lain penggunaan reference
    let a = String::from("Just another reference");
    let r1 = &a;
    let r2 = &a;
    println!("r1 = {}, r2 = {}", r1, r2);

    //collab mut & unmut reference
    let mut b = String::from("this is collab mut and unmut reference");
    let q1 = &b;
    println!("{}", q1);

    let q2 = &mut b;
    println!("{}", q2);

    let sample = "slicing word";
    let word = first_word(&sample);

    println!("{}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn gives_ownership() -> String {
//     let some_str = String::from("world");
//     some_str
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn change(some_str_str: &mut String) {
    some_str_str.push_str(" Dude!");
    println!("{}", some_str_str);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
