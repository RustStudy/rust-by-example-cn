use std::mem;

// 该函数借用一个slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 固定大小的数组 (类型签名有点多余)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有的元素都可以被初始化为相同的值(数组大小为500，值都是0)
    let ys: [i32; 500] = [0; 500];

    // 索引以 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组大小
    println!("array size: {}", xs.len());

    // 数组是被分配到栈（stack）上的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组会自动以slice类型被借用
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices可以选取数组的某一段
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 出界索引产生恐慌（panic）
    println!("{}", xs[5]);
}
