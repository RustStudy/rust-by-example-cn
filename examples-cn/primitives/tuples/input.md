元组(tuple)是一组不同类型的集合。元组由圆括号和具有类型签名值组成，
`(T1, T2, ...)`, 其中 `T1`, `T2` 是它成员的类型.函数可以用元组返回多个值，因为元组可以持有任意数量的值。

{tuples.play}

### Activity

 1. *Recap*: Add the `fmt::Display` trait to the Matrix `struct` in the above example,
    so that if you switch from printing the debug format `{:?}` to the display
    format `{}`, you see the following output:
```
( 1.1 1.2 )
( 2.1 2.2 )
```
    You may want to refer back to the example for [print display][print_display].
 2. Add a `transpose` function using the `reverse` function as a template, which
    accepts a matrix as an argument, and returns a matrix in which two elements
    have been swapped. For example:
```
println!("Matrix:\n{}", matrix)
println!("Transpose:\n{}", transpose(matrix))
```
results in the output:
```
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
```

[print_display]: /hello/print/print_display.html
