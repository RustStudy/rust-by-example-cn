我们已经见识过 *格式化字符串（format string）* 了:

* `format!("{}", foo)` -> `"3735928559"`
* `format!("0x{:X}", foo)` ->
  [`"0xDEADBEEF"`][deadbeef]
* `format!("0o{:o}", foo)` -> `"0o33653337357"`

同样的变量 (`foo`) 通过 *参数类型* 能被格式化为不同的格式。参数类型: `X` vs `o` vs *未指定（unspecified）*.

格式化的功能是通过traits实现的，每个参数类型都有一个trait。最通用的格式化trait是`Display`，它处理的参数类型是 *未指定（unspecified）*: `{}`。

{show.play}

You can view a [full list of formatting traits][fmt_traits] and their argument
types in the [`std::fmt`][fmt] documentation.

### Activity

为上面的`Color`结构体添加一个`fmt::Display` trait实现，以便于能输出下列结果:

```
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```

两点提示:

 * You [may need to list each color more than once][argument_types],
 * You can [pad with zeros to a width of 2][fmt_width] with `:02`.

### 更多参考
[`std::fmt`][fmt]

[argument_types]: http://doc.rust-lang.org/std/fmt/#argument-types
[deadbeef]: https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values
[fmt]: http://doc.rust-lang.org/std/fmt/
[fmt_traits]: http://doc.rust-lang.org/std/fmt/#formatting-traits
[fmt_width]: http://doc.rust-lang.org/std/fmt/#width
