/// 宏说Rust一组相关特性的称谓
/// 使用 macro_rules! 构建的声明宏
/// 三种过程宏
/// - 自定义#[derive]宏，用于struct或enum，可以为其指定随derive属性添加的代码
/// - 类似属性的宏，在任何条目上添加自定义属性
/// - 类似函数的宏，看起来像函数，对其指定参数的token进行操作

// 声明宏
#[macro_export]
macro_rules! myvec {
    ( $($x:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    println!("Hello, world!");

    let a = myvec![1, 2, 3];
    println!("{}", a[0]);
}
