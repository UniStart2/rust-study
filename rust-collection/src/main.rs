mod string_test {

    // Rust核心语言层面，只有一个字符串类型：字符串切片(&str)
    // 字符串字面值也是字符串切片
    pub fn test_str() {
        let str = "Hello, Rust";
        println!("{:?}", str);
    }

    // String类型来自标准库
    pub fn test_string() {
        let mut s1: String = "constant ".to_string();
        let s2: String = String::from("static ");

        let new_str = "None ";
        s1.push_str(new_str);
        println!("{:?}", s1);
        println!("{:?}", new_str);

        let s3 = s1 + &s2;

        for bytes in s3.bytes() {
            print!("{} ", bytes);
        }

        for ch in s3.chars() {
            print!("{}", ch);
        }

        let s5 = &new_str[0..2];
        println!("\n{}", s5);
    }
}

mod vec_test {
    pub fn test_vec() {
        // Vector
        let mut _v1: Vec<i32> = Vec::new();

        let _v2 = vec![1, 2, 3];

        _v1.push(1);
        _v1.push(2);
        _v1.push(3);

        println!("{:?}", _v1.get(0));

        println!("_v2: {:?}", _v2);

        for i in _v1 {
            // *i += 100;
            println!("{:?}", i);
        }
    }
}

mod hash_map_test {
    use std::collections::HashMap;

    pub fn test_hashmap() {
        let mut hash: HashMap<&str, i32> = HashMap::new();
        hash.insert("key1", 1);
        hash.insert("key2", 2);
        hash.insert("key3", 3);

        let mut op1 = hash.entry("key3");

        println!("{:?}", op1);

        for (k, v) in hash {
            println!("key: {}, value: {}", k, v);
        }

        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let p = map.entry(word).or_insert(0);
            *p += 1;
        }
        println!("{:#?}", map);
    }
}

#[cfg(test)]
mod slice_tests {
    use std::fmt;

    #[test]
    fn test_one() {
        let arr = [1, 2, 3, 4, 5];
        let vec = vec![1, 2, 3, 4, 5];
        let s1 = &arr[..2];
        let s2 = &vec[..2];
        println!("s1: {:?}, s2: {:?}", s1, s2);

        // &[T] 和 &[T] 是否相等取决于长度和内容是否相等
        assert_eq!(s1, s2);
        // &[T] 可以和 Vec<T>/[T;n] 比较，也会看长度和内容
        assert_eq!(&vec[..], arr);
        assert_eq!(&arr[..], vec);
    }

    #[test]
    fn test_two() {
        let v = vec![1, 2, 3, 4];

        // Vec实现了 Deref，&Vec<T> 会自动解引用为 &[T]，符合接口定义
        print_slice(&v);
        print_slice(&v[..]);

        // &Vec<T> 支持AsRef<[T]>
        print_slice1(&v);
        print_slice1(&v[..]);

        // Vec<T> 也支持 AsRef<[T]>
        print_slice1(v);

        let arr = [1, 2, 3, 4];
        // 数组虽没有实现 Deref，但它的解引用就是&[T]
        print_slice(&arr);
        print_slice(&arr[..]);

        print_slice1(&arr);
        print_slice1(&arr[..]);
        print_slice1(arr);
    }

    fn print_slice<T: fmt::Debug>(s: &[T]) {
        println!("{:?}", s);
    }

    fn print_slice1<T, U>(s: T)
    where
        T: AsRef<[U]>,
        U: fmt::Debug,
    {
        println!("{:?}", s.as_ref());
    }

    #[test]
    fn test_three() {
        use std::ops::Deref;

        fn inner() {
            let mut v1 = vec![1, 2, 3, 4];
            v1.push(5);
            println!("cap should be 8: {}", v1.capacity());

            // 从 Vec<T> 转换成 Box<[T]>，此时会丢弃多余的 capacity
            let b1 = v1.into_boxed_slice();
            let mut b2 = b1.clone();

            let v2 = b1.into_vec();
            println!("cap should be exactly 5: {}", v2.capacity());

            assert!(b2.deref() == v2);

            // Box<[T]> 可以更改其内部数据，但无法 push
            b2[0] = 2;
            // b2.push(6);
            println!("b2: {:?}", b2);

            // 注意 Box<[T]> 和 Box<[T; n]> 并不相同
            let b3 = Box::new([2, 2, 3, 4, 5]);
            println!("b3: {:?}", b3);

            // b2 和 b3 相等，但 b3.deref() 和 v2 无法比较
            assert!(b2 == b3);
            // assert!(b3.deref() == v2);
        }

        inner();
    }
}

fn main() {
    vec_test::test_vec();

    hash_map_test::test_hashmap();

    string_test::test_string();
}
