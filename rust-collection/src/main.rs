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
        let mut op2 = hash.entry("key4");

        for (k, v) in hash {
            println!("key: {}, value: {}", k, v);
        }
    }
}

mod FileTest {
    use std::fs::File;

    pub fn test_open_file() {
        let file = File::open("hello.txt");
        match file {
            Ok(file) => {
                println!("打开文件成功！");
            }
            Err(err) => {
                panic!("打开文件失败：{}", err);
            }
        }
    }
}

fn main() {
    VecTest::test_vec();

    HashMapTest::test_hashmap();

    FileTest::test_open_file();
}
