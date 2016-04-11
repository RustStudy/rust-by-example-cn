fn main() {
    // 一般来说， `{}` 会自动替换任何参数。

    println!("{} days", 31);

    // 没有后缀，31默认是i32。你可以使用后缀来改变31的类型。

    // 有多种可选模型。位置参数也可用。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 以及命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // 特殊格式可以在`:`后面指定。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你也能通过指定宽度来对文本进行右对齐。
    // 将会输出 "     1"
    // "1"前面有5个空格。
    println!("{number:>width$}", number=1, width=6);

    // 你也可以填充0,会输出"000001".
    println!("{number:>0width$}", number=1, width=6);

    // 甚至还可以检查参数个数是否正确
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ 添加确实的参数: "James"

    // 创建一个包含了`i32`类型参数的名为 `Structure`的结构体（structure
    struct Structure(i32);

    // 然后，诸如这样自定义类型的结构体则需要更复杂的处理。
    // 该行代码将无法执行。
    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ 注释掉这行。
}
