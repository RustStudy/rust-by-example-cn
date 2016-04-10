想要使用`std::fmt`格式化`traints`的所有类型都需要可打印(printable)的实现。
自动实现仅提供给诸如`std`库里的类型。其他类型则*必须*想办法手工实现。

`fmt::Debug` `trait` 使用起来非常简单。*所有* 类型都可以
`derive` (自动创建) `fmt::Debug` 的实现. `fmt::Display`并非一定要手工实现。

```rust
// 该结构体不能被`fmt::Display`和 `fmt::Debug`打印
struct UnPrintable(i32);

// `derive`属性会用`fmt::Debug`为该`结构体`自动创建必须的可打印实现。
#[derive(Debug)]
struct DebugPrintable(i32);
```

`std`标准库类型使用`{:?}`也可以自动实现可打印:

{debug.play}

所以`fmt::Debug` 肯定可以实现可打印，但是会失去一些优雅。手工实现`fmt::Display` 会解决这个问题。

### 更多参考

[attributes][attributes], [`derive`][derive], [`std::fmt`][fmt],
以及 [`struct`][structs]

[attributes]: http://doc.rust-lang.org/reference.html#attributes
[derive]: /trait/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[structs]: /custom_types/structs.html
