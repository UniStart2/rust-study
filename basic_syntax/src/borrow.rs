// rust 中的借用语义
pub fn testBorrowSyntax() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}

//  借用的生命周期及其约束（借用不能超过值的生命周期）
// fn testlifetime() {
//     // error: 生命周期更长的main()函数变量r，
//     // 引用了生命周期更短的 local_ref()函数里的局部变量
//     let r = local_ref();
//     println!("r: {:p}", r);
// }

// fn local_ref<'a>() -> &'a i32 {
//     let a = 42;
//     &a
// }

fn testHeapRefStack() {
    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);
    println!("data: {:?}", data);
}

// Rust为了保证内存安全，对可变引用的使用也做了严格的约束：
// 在一个作用域内，仅允许一个活跃的可变引用。所谓活跃，就是真正被使用来修改数据的可变引用，
//  如果只是定义了，却没有使用或者当作只读引用使用，不算活跃。
// 在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。
pub fn test_practice() {
    let mut arr = vec![1, 2, 3];
    // cache the last item
    let last = arr.last();
    // consume previously stored last item
    println!("last: {:?}", last);
    arr.push(4);
}
