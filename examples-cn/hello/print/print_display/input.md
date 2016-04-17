`fmt::Debug` 看上去相当紧凑干净，它们实现定义输出很方便。它实际是通过`{}`打印标识实现的[`fmt::Display`][fmt]. 实现过程类似于下列代码:

```rust
// 通过`use`导入`fmt`模块
use std::fmt;

//  定义将实现`fmt::Display`的结构体.
// 这是一个简单包含`i32`类型参数的元组（tuple）结构体 `Structure`.
struct Structure(i32);

// 为了使用`{}`, 一定要为该类型实现`fmt::Display` trait

impl fmt::Display for Structure {
    // 该trait必须显式指定`fmt`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 把输出流写入到第一个元素中: `f`.
        // 返回`fmt::Result`标示操作是否成功
        // 注意`write!`的用法和`println!`非常类型。
        write!(f, "{}", self.0)
    }
}
```

`fmt::Display` 可能比`fmt::Debug`更干净，但是这凸显了`std`的一个问题. 该如何显示不明确的类型呢？
例如，如果`std` 库为所有的`Vec<T>`分别实现了一个单独的类型， 该如何显示？只显示下列其中一种，还是两种都显示？

* `Vec<path>`: `/:/etc:/home/username:/bin` (用`:`分隔)
* `Vec<number>`: `1,2,3` (`,`分隔)

No, because there is no ideal style for all types and the `std` library
doesn't presume to dictate one. `fmt::Display` is not implemented for `Vec<T>`
or for any other generic containers. `fmt::Debug` must then be used for these
generic cases.

This is not a problem though because for any new *container* type which is
*not* generic,`fmt::Display` can be implemented.

{display.play}

So, `fmt::Display` has been implemented but `fmt::Binary` has not, and
therefore cannot be used. `std::fmt` has many such [`traits`][traits] and
each requires its own implementation. This is detailed further in
[`std::fmt`][fmt].

### Activity

After checking the output of the above example, use the `Point2` struct as
guide to add a Complex struct to the example. When printed in the same
way, the output should be:
```
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

### See also

[`derive`][derive], [`std::fmt`][fmt], [macros], [`struct`][structs],
[`trait`][traits], and [use][use]

[derive]: /trait/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: /macros.html
[structs]: /custom_types/structs.html
[traits]: /trait.html
[use]: /mod/use.html
