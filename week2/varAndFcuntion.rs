// 参考https://rustwiki.org/zh-CN/book/ch03-02-data-types.html

use std::io;

fn main() {
    // 变量
    let mut x = 1; // mut可变变量 定义了要被使用
    println!("x is {}", x);
    x = 5;
    println!("x is {}", x);

    // 常量用const，且必须标注类型
    const MAX_POINTS: u32 = 100_000; // 加_提高可读性
    println!("MAX_POINTS is {}", MAX_POINTS);

    // 变量遮蔽, 和mut不同因为采用了新的let，相当于重定义了，所以可以是新的类型
    let y = 1;
    // let y = y + 1;
    let y = "hh";
    println!("y is {}", y);


    // 数据类型(rust是静态类型语言)
    let guess: u32 = "42".parse().expect("not a number!");
    println!("guess is {}", guess);
    // （1）整型 u(8-128,size)
    // （2）浮点型 f(64 32)
    let f1 = 1.01; // f64
    let f2: f32 = 2.0;
    println!("f1 is {}, f2 is {}", f1, f2);


    // （3）布尔型
    let bool = false;
    println!("bool is {}", bool);
    // （4）字符型 unicode值
    let c1 = "a";
    println!("c1 is {}", c1);

    // (5)复合类型 其他多个类型组成的一个类型
    // 元组
    let yuan = (1.11, 4, "不是");
    let (yuan1, yuan2, yuan3) = yuan; // 解构赋值
    println!("yuan1 is {}", yuan1);
    println!("yuan2 is {}", yuan2);
    println!("yuan3 is {}", yuan3);
    // 数组 (每个元素必须是同一类型)
    let arr = [1, 3, 4];
    // 数组读取
    // println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("failed to read line");

    let index: usize = index.trim().parse().expect("index entered was not a number");
    let ele = arr[index];
    println!("ele is {}", ele);

    // 函数调用
    let res = test(3);
    println!("res is {}", res);


    // 控制流程
    let num = 5;
    if num > 10 {
        println!("{} big than 10", num);
    }
    else {
        println!("{} small than 10", res);
    }
    let condition = num > 10;
    let num1 = if condition { // 有点类似三元表达式
        num
    }
    else {
        0
    };
    println!("num1 is {}", num1);

    let mut counter = 0;
    let loopResult = loop {
        if counter < 10 {
            counter = counter+1;
        }
        else {
            break counter;
        }
    };
    println!("loopResult is {}", loopResult);

    // while
    while counter < 20 {
        counter = counter + 2;
    }
    println!("counter is {}", counter);


    // for in
    for element in arr.iter() {
        println!("the value is: {}", element);
    }
}

// 函数
fn test(a: i32) -> i32 {
    return a + 1;
}