mod unsafe_test {
    pub extern "C" fn func() {
        println!("call Rust function pass by C");
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
            let ptr1 = &number as *const i32;
            let ptr2 = &mut number as *mut i32;

            println!(
                "ptr1 addr: {:p}, ptr1 val: {:p}, ref val: {}\n\
                ptr2 addr: {:p}, ptr2 val: {:p}, ref val: {}",
                &ptr1, ptr1, *ptr1, &ptr2, ptr2, *ptr2,
            )
        }
    }
}

fn main() {
    use unsafe_test::test;

    test();
}
