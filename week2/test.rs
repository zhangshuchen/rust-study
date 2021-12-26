fn main() {
    let res1 = degreesCelsiusToFahrenheit(4);
    println!("4 degress is {} Fahrenheit", res1);
    let res2 = frib1(4);
    println!("res2 is {}", res2);
    // let res3 = 
}

// 相互转换摄氏与华氏温度。
fn degreesCelsiusToFahrenheit(degrees: u32) -> f32{
    let a: f32 = degrees as f32;
    // let b: f32 = f32::from(32);
    return (a * 1.8) + 32u32 as f32;
}

// 生成 n 阶斐波那契数列
fn frib1(n: usize) -> usize{
    if n == 0 || n == 1 {
        return n;
    }
    else {
        return frib1(n - 1) + frib1(n - 2);
    }
}

// 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）
