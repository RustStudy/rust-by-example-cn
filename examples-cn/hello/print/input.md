格式化输出是由定义于[`std::fmt`][fmt]中的一系列[`宏`][macros] 来进行处理的，包括下面所列的部分宏：

* `format!`: 格式化 [`字符串（String）`][string]。
* `print!`: 类似于`format!`，但文本只打印到控制台。
* `println!`: 类似于`print!`，但是会追加新行。

跟所有文本解析风格相似，rust的亮点在于格式的正确性会在编译期进行检查。

{print.play}

[`std::fmt`][fmt] 包含了很多控制文本显示的[`traits`][traits] 。下面列出两个重要的基本形式:

* `fmt::Debug`: 使用 `{:?}` 标记，以便于调试。
* `fmt::Display`: 使用`{}` 标记，以优雅、用户友好的方式来格式化文本。

这里使用`fmt::Display` 是因为标准库提供了这种类型的很多实现。要打印自定义类似的文本，则需要做更多的工作。

### Activities

 * 修改上方代码中的两个问题 (看FIXME) ，让代码正确运行。
 * 添加`println!` 宏来输出: `Pi is roughly 3.143`, 用22/7来生成Pi的值。 (提示：你可能需要检查[`std::fmt`][fmt]文档来设置显示的小数精度)

### 更多文档

[`std::fmt`][fmt], [`macros`][macros], [`struct`][structs],
以及 [`traits`][traits]

[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: /macros.html
[string]: /std/str.html
[structs]: /custom_types/structs.html
[traits]: /trait.html
