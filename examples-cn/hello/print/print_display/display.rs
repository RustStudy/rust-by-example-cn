use std::fmt; // 导入 `fmt`

// derive `Debug`为了和使用`Display`形成对比
#[derive(Debug)]
struct MinMax(i64, i64);

// 为 `MinMax`实现`Display`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 用`self.number`引用每一个位置的数据
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 定义一个用于比对的结构体
#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

// 同样，为Point2实现Display
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2 { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error.
    // `Debug` 和 `Display`都能被实现，但是`{:b}`需要实现`fmt::Binary`
    //下面这行将不会被执行
    // println!("What does Point2D look like in binary: {:b}?", point);
}
