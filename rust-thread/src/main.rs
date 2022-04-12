// Rust中的线程创建方式
#[cfg(test)]
mod thread_test {
    use std::{mem::take, thread, time::Duration};

    #[test]
    fn test() {
        let v = 1;

        let task1 = thread::spawn(move || {
            let t = Duration::new(1, 0);
            thread::sleep(t);
            println!("v: {}", v);
        });

        let res = task1.join(); //主线程等待其线程结束
        println!("task1 result: {:?}", res);
        println!("thread name: {}", thread::current().name().unwrap());
    }
}

// Rust中的消息传递
// 创建chnannel，Rust中使用的是multiple provider-single consumer模式
mod channel_test {
    use std::f32::consts::E;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let (tx, rx) = mpsc::channel();
        let tx2 = mpsc::Sender::clone(&tx);

        let t1 = thread::spawn(move || {
            let arr: [&str; 3] = ["t1: 1", "t1: 2", "t1: 3"];

            for ele in arr {
                tx.send(ele).unwrap();
                thread::sleep(Duration::from_secs(3));
            }
        });

        let t2 = thread::spawn(move || {
            let arr: [&str; 3] = ["t2: a", "t2: b", "t2: c"];

            for ele in arr {
                tx2.send(ele).unwrap();
                thread::sleep(Duration::from_secs(3));
            }
        });

        for received in rx {
            println!("receive: {}", received);
        }
    }
}

#[cfg(test)]
mod mutex_test {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    #[test]
    fn test() {
        let mutex = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let mutex = Arc::clone(&mutex);
            let t = thread::spawn(move || {
                // 尝试获取锁
                let mut num = mutex.lock().unwrap();

                *num += 1;
            });

            handles.push(t);
        }

        for thread in handles {
            thread.join().unwrap();
        }

        println!("result: {}", mutex.lock().unwrap());
    }
}

fn main() {}
