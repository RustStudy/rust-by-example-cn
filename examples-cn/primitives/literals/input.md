整数 `1`, 浮点数 `1.2`, 字符 `'a'`, 字符串 `"abc"`, 布尔值 `true`
以及单元（unit）类型 `()` 都能被用字面量表示。

整数可以使用前缀`0x`, `0o` 或 `0b`分别表示十六进制、八进制或二进制.

下划线可以用在数字字面量中提升可读性。比如
`1_000` 等同于 `1000`,   `0.000_001` 等同于`0.000001`.

我们需要告诉编译器使用的字面量类型。现在，我们将使用 `u32`后缀来标明该字面量是无符号32位整数，并且`i32`后缀标明符号型32位整数。

[ Rust中的][rust op-prec] 操作符及其优先级都类似于其他[类C语言][op-prec].。

{literals.play}

[rust op-prec]: http://doc.rust-lang.org/reference.html#operator-precedence
[op-prec]: https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages
