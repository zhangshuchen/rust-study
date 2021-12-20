use std::io; // 输入输出库
use rand::Rng; // 生成随机数
use std::cmp::Ordering; // 比较随机数据大小

fn main() {

    println!("Guess the number");

    //
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("the secret_number is: {}", secret_number);
    loop {

        println!("please input your guess.");
        // 创建一个变量
        let mut guess = String::new(); // 静态类型的静态方法

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line"); // Result 类型 是枚举

        println!("you guess: {}", guess);
        // 类型变换的时候可以遮蔽错误
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // _是一个通配符
        };
        // let x = 5;
        // let y = 6;
        // println!("x = {} and y = {}", x, y);

        // guess 是数值
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
