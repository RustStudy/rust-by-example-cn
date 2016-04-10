任何程序都需要注释（comment），Rust甚至支持几种不同的格式:

* *一般注释（Regular comments）* 会被编译器忽略掉:
 - `// Line comments which go to the end of the line.`
 - `/* Block comments which go to the closing delimiter. */`
* *文档注释（Doc comments）* 会被解析为HTML[文档][docs]:
 - `/// Generate library docs for the following item.`
 - `//! Generate library docs for the enclosing item.`

{comment.play}

### 更多文档:

[Library documentation][docs]

[docs]: /meta/doc.html
