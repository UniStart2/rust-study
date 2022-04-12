use std::{mem::take, thread, time::Duration};

fn main() {
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
