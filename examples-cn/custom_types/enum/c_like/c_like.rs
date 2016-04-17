// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// 使用隐式辨别值(discriminator)(从 0 开始)
enum Number {
    Zero,
    One,
    Two,
}

// 使用显式指定辨别值(discriminator)
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums`能被作为整数计算.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    // 这里可以尝试使用`use`关键字来省略'Color::'
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
