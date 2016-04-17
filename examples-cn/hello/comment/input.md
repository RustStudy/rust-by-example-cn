任何程序都需要注释（comment），Rust甚至支持几种不同的格式:

* *一般注释（Regular comments）* 会被编译器忽略掉:
 - `// 行注释，对整行有效.`
 - `/* 块注释，在相应的界定符内有效 */`
* *文档注释（Doc comments）* 会被解析为HTML[文档][docs]:
 - `/// 为下面代码生成库文档`
 - `//! 为本行生成库文档`

{comment.play}

### 更多文档:

[Library documentation][docs]

[docs]: /meta/doc.html
