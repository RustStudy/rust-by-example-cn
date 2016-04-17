fn main() {
    // 变量能被类型标注
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 一般标注
    let an_integer   = 5i32; // 后缀标注 annotation

    // 不标注的话就用默认值
    let default_float   = 3.0; // 默认为`f64`
    let default_integer = 7;   // 默认为`i32`

    let mut mutable = 12; // 可变（Mutable） `i32`.

    // Error! 该变量的类型不能被改变
    mutable = true;
}
