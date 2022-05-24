/// unsafe 可以执行的四个动作
/// - 1、解引用原始指针
/// - 2、调用unsafe函数或方法
/// - 3、访问或修改可变的静态变量
/// - 4、实现 unsafe trait
/// unsafe并没有关闭借用检查或停用其他安全检查

mod unsafe_test {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    pub unsafe trait UnsafeMyTrait {
        fn print_str(str: &str) {
            println!("str: {}", str);
        }
    }

    pub fn test() {
        unsafe impl UnsafeMyTrait for str {}

        unsafe {
            let mut number: i32 = 5i32;
            // 原始指针
            // 可变的：*mut T
            // 不可变的：*const T
            // 注意：*号是类型名的一部分
            let ptr1 = &number as *const i32;
            let ptr2 = &mut number as *mut i32;
            *ptr2 = 2;

            println!(
                "ptr1 addr: {:p}, ptr1 val: {:p}, ref val: {}\n\
                ptr2 addr: {:p}, ptr2 val: {:p}, ref val: {}",
                &ptr1, ptr1, *ptr1, &ptr2, ptr2, *ptr2,
            );

            let ptr3: *const i32 = &3;
            println!(
                "ptr3 addr: {:p}, ptr3: {:p}, ref value: {}",
                &ptr3, ptr3, *ptr3
            );

            println!("Absolute value of -3 according to C: {}", abs(3));
        }
    }
}

fn main() {
    use unsafe_test::test;

    test();
}
