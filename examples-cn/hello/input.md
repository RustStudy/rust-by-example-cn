来段Hello World代码

{hello.play}


`println!`是一个[*宏（macro）*][macros] ，用于在控制台打印文本。


可以使用Rust编译器`rustc`来生成二进制文件。

```
$ rustc hello.rs
```

`rustc` 会生成二进制可执行文件 `hello`

```
$ ./hello
Hello World!
```

### Activity

点击上面的'Run'能看到期望的输出结果。然后，请用`println!`宏添加一行代码以便于输出下面的结果：

```
Hello World!
I'm a Rustacean!
```

[macros]: ./macros.html
