
// 切片slice https://rustwiki.org/zh-CN/book/ch04-03-slices.html
fn main() {
    // 切片 slice
    let mut s = String::from("hello world");
    println!("s is {}", s);

    let word = first_word(&s);

    println!("word is {}", word);
    s.clear();

    // 字符串slice 部分string的引用
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // println!("hello is {}", hello);

    // println!("world is {}", world);

    // 字符串面值就是slice
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}