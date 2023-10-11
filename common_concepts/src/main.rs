#[allow(unused)]

fn main() {
    #[allow(dead_code)]
    // mutable and immutable 
    let _x = 10;
    let mut y = 20;

    // use const
    const LOWEST_PRICE: u32 = 1_000_000;

    //parse type data
    let a = 10;
    let a = "ten";

    let i = 10;
    let f = 1.1;
    let valid = true;
    let invalid: bool = false;

    // Char use single quotes dan string use double quotes
    let c = 'C';
    let s = "S";

    // tuples and array

    //tuples can with different types
    let tup = (1, 1.4, 100);
    let (x,y,z) = tup;
    let first = tup.0;

    let tup_2 = ();

    // Array must same type
    let array_a = [1,2,3,4];
    let array_a = [2,5]; // => [2,2,2,2,2]
    let array_b = ['A','B','C'];
    let first_array = array_a[0];

    // function concept
    println!("Hello this is function");
    example_function();
    parse_parameter(2, 3);

    let res_let_try = let_try(2, 3);
    println!("{}", res_let_try);

    let num = 3;

    if num < 10 {
        println!("Condition true");
    }
    else if num > 15 && num % 3 == 0 {
        println!("Its amazing");
    }
    else {
        println!("Condition false");
    }

    // if else with another syntax
    let cond = true;
    let num_2 = if cond {5} else {6};

    // loop
    let mut counter = 0;
    let res_counter = loop {
        counter += 1;
        println!("{}", counter);

        if counter == 5 {
            break;
        }
    };

    // while logic
    let mut num_while = 3;

    while num_while != 0{
        println!("{}", num_while);
        num_while -= 1;
    }
    println!("bye");

    // for
    let num_for = [1,2,3];

    for element in num_for{
        println!("The value on num_for is {}", element);
    }

    // another for
    for number_another_for in 1..=5{
        println!("{}", number_another_for);
    }
    println!("bye");
}

fn example_function() {
    println!("This is example function");
}

fn parse_parameter(x: i32, y: i32) {
    println!("this is parsing parameter: {},{}", x,y);
}

fn let_try(x:i32,y:i32) -> i32 {
    x+y
}

// if condition
