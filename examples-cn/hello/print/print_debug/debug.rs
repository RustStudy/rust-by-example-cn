// 从`fmt::Debug` 中导出（derive）实现。
// `Structure`是包含了 单个`i32`类型参数的结构体。
#[derive(Debug)]
struct Structure(i32);

// 置于结构体`Deep`中的`Structure`也是可打印的。
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // `{:?}`用法于 `{}`相似。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` 是可打印的!
    println!("Now {:?} will print!", Structure(3));

    // 使用`derive`的问题是对结果如何显示没有控制权。
    // 假如我这里想显示`7`呢?
    println!("Now {:?} will print!", Deep(Structure(7)));
}
