// 所有权：参考https://rustwiki.org/zh-CN/book/ch04-01-what-is-ownership.html
fn main() {
    // 可变可增长的，堆上分配一块编译时大小未知的内存来存放内容
    let mut str = String::from("hello");
    str.push_str(", world");
    println!("str is {}", str);
    // 内存再拥有他的变量离开作用域的时候就会释放
    // 变量离开作用域，rust会为我们调用一个特殊的函数叫drop
    // 二次污染，rust认为被复制的变量不再使用 （移动而不是浅copy）
    // （1）移动
    let s2 = str; // str不再使用
    // println!("str is {}", str);

    // （2）克隆
    let source = String::from("hello");
    let target = source.clone();
    println!("source is {}", source);
    println!("target is {}", target);

    takes_ownership(source);
    // println!("source is {}", source); // 被移动了不能访问

    let num = 2;
    makes_copy(num);
    println!("num is {}", num); // 栈上copy，所以可以访问


    // 返回值
    let s1 = gives_ownership(); // 内部被转移给s1

    let s2 = String::from("hello s2");

    let s3 = takes_and_gives_back(s2); // s2被转移给S3

    // 总结，变量的所有权 总是 当值复制给另一个值时被 移动， 当离开作用域时被drop


    // 引用与借用
    // 引用, 引用并没有所有权，所以不会改变所有权，且引用的值不能被修改
    let s11 = String::from("hello hshfhd");

    let len = calculate_length(&s11);
    println!("{}", len);

    // 可变引用
    let mut s22 = String::from("hello hshfhd");
    let r1 = &mut s22;
    // let r2 = &mut s22; // 不允许可变俩次，防止数据竞争

    // 悬垂引用 （指向的内存可能已经被分配给其他的所有者）
    dangle();

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: u32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
    // 这里 s 离开作用域并被丢弃。其内存被释放。这时指针指向了无效内存，报错
    s // 直接返回s交出所有权即可
}
