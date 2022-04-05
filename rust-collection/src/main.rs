mod StringTest {

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

mod VecTest {
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

mod HashMapTest {
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

fn main() {
    VecTest::test_vec();

    HashMapTest::test_hashmap();

    StringTest::test_string();
}
