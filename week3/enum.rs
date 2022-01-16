// 枚举和模式匹配
//  代数数据类型


// 定义枚举
enum IpAddressKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddressKind,
    address: String
}

//
enum IpAddressKind1 {
    // 可以将任意数据类型放入枚举成员中
    V4(String),
    V6(String)
}

enum Option<T> {
    Some(T),
    None()
}

fn main() {
    // 都是IpAddressKind 类型的
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    let home = IpAddr {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1")
    }

    let lookback = IpAddr {
        kind: IpAddressKind::V6,
        address: String::from("::1")
    }

    let home1 = IpAddressKind1::V4(String::from("127.0.0.1"))
    let lookback1 = IpAddressKind1::V6(String::from("::1"))

    // option 枚举类，弥补不支持null引用的空白
    let some_number = Some(4)
    let some_string = Some("a string")

    let absent_number: Option<i32> = None; // 编译器只通过None无法知道类型

}