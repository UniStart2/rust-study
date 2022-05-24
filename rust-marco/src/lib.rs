use proc_macro;

// 自定义derive宏
pub trait HelloMacro {
    fn hello_macro() {
        println!("hello macro");
    }
}
