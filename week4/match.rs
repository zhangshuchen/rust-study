// https://rustwiki.org/zh-CN/book/ch06-02-match.html
// match 控制流程

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    // println!("res is {}", value_in_cents(Coin::Nickel));
    // value_in_cents()

    // 绑定值模式
    value_in_cents1(Coin::Quarter(UsState::Alabama));

    // 匹配Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six is {:?}", six);

    // 通配符
    let some_u8_val = 0u8;
    println!("six is {:?}", wildcard_character(some_u8_val));

    // if let
    if_let(8);
}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

fn value_in_cents1(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) =>  Some(i + 1)
    }
}

fn wildcard_character(val: u8) {
    match val {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn if_let(val: u32) {
    // 匹配match的某一个分支
    if let val = 8 {
        println!("eight");
    }
}