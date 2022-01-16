
// use std::cmp::Ordering
// use std::io;
// 合并
use std::{cmp::Ordering, io}

use std::collections::*

fn serve_order() {}

mod back_of_house {
    // 公有的结构体和枚举
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // 引用父级函数
        super::serve_order();
    }
    fn cook_order() {

    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
        }
        fn seat_at_table() {

        }
    }

    mod serving {
        fn task_order() {

        }
        fn server_order() {

        }
        fn task_payment() {

        }
    }
}

use crate::front_of_house::hosting

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();


    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("toast");
    println!("I'd like {} toast please", meal.toast);

    // 
    hosting::add_to_waitlist();
}