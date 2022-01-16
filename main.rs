// https://rustwiki.org/zh-CN/book/ch08-00-common-collections.html
// 常见集合

#[derive(Debug)]
enum SpreedSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    // （1）vector
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(5);
    // 当向在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中
    v.push(6);

    let v1 = vec![1, 2, 3];

    let _third: &i32 = &v[2];

    match v1.get(2) {
        Some(_third) => println!("The third element is {}", _third),
        None => println!("There is no third element.")
    }

    // 遍历
    for i in &mut v {
        println!("{}", i);
    }


    // 利用枚举扩展类型
    let row = vec![
        SpreedSheetCell::Int(32),
        SpreedSheetCell::Text(String::from("blue")),
        SpreedSheetCell::Float(10.12)
    ];
    println!("{:?}", row);

    // （2）string
    // 作为字节的集合外加一些方法实现的
    // String 是一个 Vec<u8> 的封装
    let mut s1 = String::from("ff"); // 通过字面量生成String
    s1.push("dhhdh");

    let s2 = "hhhh";

    let s3 = s1 + &s2; // 这里默认调用了add
    println!("s3 is {}", s3);

}