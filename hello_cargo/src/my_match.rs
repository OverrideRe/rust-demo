#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => {
            println!("Lucky Nickel!");
            5
        },
        Coin::Dime => {
            println!("Lucky Dime!");
            10
        },
        Coin::Quarter(usState) => {
            println!("Lucky Quarter! usState : {:#?}", usState);
            25
        }
    }
}

fn match_test() {
    let coin = Coin::Penny;
    println!("result : {}", value_in_cents(coin));
    
    let coin = Coin::Quarter(UsState::Alaska);
    println!("result : {}", value_in_cents(coin));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("some match. result : {}", i + 1);
            Some(i + 1)
        }
    }
}

fn some_match(x: u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("no match")
    }
}

fn if_let_test(x: u8) {
    if let 3 = x {
        println!("yes");
    } else {
        println!("no");
    }
}


fn main() {
    match_test();
    println!("---------------");
    let res = plus_one(Option::Some(1));
    println!("---------------");
    some_match(3);
    some_match(5);
    println!("---------------");
    if_let_test(2);
    if_let_test(3);
}