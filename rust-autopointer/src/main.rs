// mod Allocator {
//     use std::alloc::{GlobalAlloc, Layout, System};

//     struct MyAllocator;

//     unsafe impl GlobalAlloc for MyAllocator {
//         unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//             let data = System.alloc(layout);
//             eprintln!("ALLOC: {:p}, size {}", data, layout.size());
//             data
//         }

//         unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//             System.dealloc(ptr, layout);
//             eprintln!("FREE: {:p}, size {}", ptr, layout.size());
//         }
//     }

//     // 使用自定义的全局内存分配器
//     #[global_allocator]
//     static GLOBAL: MyAllocator = MyAllocator;

//     #[allow(dead_code)]
//     pub struct Matrix {
//         // 使用不规则的数字如 505 可以让 dbg! 的打印很容易分辨出来
//         data: [u8; 505],
//     }
//     impl Default for Matrix {
//         fn default() -> Self {
//             Self { data: [0; 505] }
//         }
//     }

//     #[test]
//     fn test() {
//         // 在这句执行之前已经有好多内存分配
//         let data = Box::new(Matrix::default());

//         // 输出中有一个 1024 大小的内存分配，是 println! 导致的
//         println!(
//             "!!! allocated memory: {:p}, len: {}",
//             &*data,
//             std::mem::size_of::<Matrix>()
//         );

//         // data 在这里 drop，可以在打印中看到 FREE
//         // 之后还有很多其它内存被释放}
//     }
// }

mod json_serialize {

    use serde::Deserialize;
    use std::borrow::Cow;

    #[derive(Debug, Deserialize)]
    struct User<'input> {
        #[serde(borrow)]
        name: Cow<'input, str>,
        age: u8,
    }

    #[test]
    fn test() {
        let input = r#"{ "name": "Tyr", "age": 18 }"#;
        let user: User = serde_json::from_str(input).unwrap();

        match user.name {
            Cow::Borrowed(x) => println!("borrowed {}", x),
            Cow::Owned(x) => println!("owned {}", x),
        }
    }
}

mod my_string {

    use std::{fmt, ops::Deref, str};

    const MINI_STRING_MAX_LEN: usize = 30;

    // MyString 里，String 有 3 个 word，供 24 字节，所以它以 8 字节对齐
    // 所以 enum 的 tag + padding 最少 8 字节，整个结构占 32 字节。
    // MiniString 可以最多有 30 字节（再加上 1 字节长度和 1字节 tag），就是 32 字节.
    struct MiniString {
        len: u8,
        data: [u8; MINI_STRING_MAX_LEN],
    }

    impl MiniString {
        // 这里 new 接口不暴露出去，保证传入的 v 的字节长度小于等于 30
        fn new(v: impl AsRef<str>) -> Self {
            let bytes = v.as_ref().as_bytes();
            // 我们在拷贝内容时一定要使用字符串的字节长度
            let len = bytes.len();
            let mut data = [0u8; MINI_STRING_MAX_LEN];
            data[..len].copy_from_slice(bytes);
            Self {
                len: len as u8,
                data,
            }
        }
    }

    impl Deref for MiniString {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            // 由于生成 MiniString 的接口是隐藏的，它只能来自字符串，所以下面这行是安全的
            str::from_utf8(&self.data[..self.len as usize]).unwrap()
            // 也可以直接用 unsafe 版本
            // unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize]) }
        }
    }

    impl fmt::Debug for MiniString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // 这里由于实现了 Deref trait，可以直接得到一个 &str 输出
            write!(f, "{}", self.deref())
        }
    }

    #[derive(Debug)]
    enum MyString {
        Inline(MiniString),
        Standard(String),
    }

    // 实现 Deref 接口对两种不同的场景统一得到 &str
    impl Deref for MyString {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            match *self {
                MyString::Inline(ref v) => v.deref(),
                MyString::Standard(ref v) => v.deref(),
            }
        }
    }

    impl From<&str> for MyString {
        fn from(s: &str) -> Self {
            match s.len() > MINI_STRING_MAX_LEN {
                true => Self::Standard(s.to_owned()),
                _ => Self::Inline(MiniString::new(s)),
            }
        }
    }

    impl fmt::Display for MyString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.deref())
        }
    }

    #[test]
    fn test() {
        let len1 = std::mem::size_of::<MyString>();
        let len2 = std::mem::size_of::<MiniString>();
        println!("Len: MyString {}, MiniString {}", len1, len2);

        let s1: MyString = "hello world".into();
        let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

        // debug 输出
        println!("s1: {:?}, s2: {:?}", s1, s2);
        // display 输出
        println!(
            "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
            s1,
            s1.len(),
            s1.chars().count(),
            s2,
            s2.len(),
            s2.chars().count()
        );

        // MyString 可以使用一切 &str 接口，感谢 Rust 的自动 Deref
        assert!(s1.ends_with("world"));
        assert!(s2.starts_with("这"));
    }
}

#[cfg(test)]
mod box_test {
    use std::{fmt::Display, ops::Deref};

    #[test]
    fn test() {
        // Box<T> 是一个只能指向堆数据的智能指针
        let b = Box::new(5);
        println!("b: {}", b);

        use List::{Cons, Nil};
        let n = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    #[derive(Debug)]
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(s: T) -> MyBox<T> {
            MyBox(s)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            // println!("drop: {:?}", self);
            std::mem::drop(self);
        }
    }

    #[test]
    fn test2() {
        let a = 2;
        let b = MyBox::new(a);

        assert_eq!(a, *b);
    }
}

#[cfg(test)]
mod rc_test {
    // Rc只能够在单线程的情况下使用
    use std::rc::Rc;

    enum RcList {
        Nil,
        Next(i32, Rc<RcList>),
    }

    #[test]
    fn test() {
        use crate::rc_test::RcList::{Next, Nil};
        let list_c = Rc::new(Next(3, Rc::new(Next(4, Rc::new(Nil)))));

        let list_a = Rc::new(Next(1, Rc::clone(&list_c)));
        let list_b = Rc::new(Next(2, Rc::clone(&list_c)));

        println!("stronge reference: {}", Rc::strong_count(&list_c));
    }
}

fn main() {}
