// struct 自定义数据类型
// 多个数组放到一个

// struct user {
//     username: String, // 需要定义为string
//     email: String,
//     sign_in_count: u64,
//     active: bool
// }

// 结构体
#[derive(Debug)] // 派生debug trait 用于调试打印
struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area2(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    // let user = build_user(String::from("cp"), String::from("rocboss"));
    // println!("user is {:?}", user);

    // 元组
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    // 结构体
    let rect1 = Rect {
        width: 30,
        height: 40
    };
    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(&rect1)
    );

    let rect2 = Rect {
        width: 30,
        height: 80
    };
    // 方法语法
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area2()
    );
}

// fn build_user(email: String, username: String) -> user {
//     user {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1
//     }
// }

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area1(rect_demo: &Rect) -> u32 {
    rect_demo.width * rect_demo.height
}
