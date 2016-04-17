// 该属性隐藏无用代码警告
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 显式使用`use`，所以无需手工指定作用域
    use Status::{Poor, Rich};
    // 为`Work`中的每个name都自动的使用了 `use`
    use Work::*;

    // 执行 `Status::Poor`.
    let status = Poor;
    // 执行`Work::Civilian`.
    let work = Civilian;

    match status {
        // 注意，这里因为上面使用了`use`，没有指定作用域，比如"Status::Rich"
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 同样没有指定作用域
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
