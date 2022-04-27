fn main() {
    // 1、匹配字面值
    let x = Some(5);
    let y = 10;
    match x {
        Some(y) => println!("Inner y = {}", y),
        _ => println!("Default case"),
    }
    println!("Outer x: {:?}, y: {}", x, y);

    // 2、匹配多个值
    let m = 1;
    match m {
        1 | 2 => {
            println!("One or two")
        }
        _ => println!("Default case"),
    }

    // 3、匹配某个范围的值
    let n = 5;
    match n {
        1..=5 => println!("One through five"),
        _ => println!("Default case"),
    }

    let ch = 'c';
    match ch {
        'a'..='z' => println!("Lowercase"),
        'A'..='Z' => println!("Uppercase"),
        _ => println!("Default case"),
    }

    // 4、利用match解构
    let p = Point { x: 31, y: 35 };
    let Point { x: c1, y: c2 } = p;
    println!("c1: {}, c2:{}", c1, c2);

    // 使用 ".." 来忽略一部分值
    let tuple = (1, 2, 3, 4, 5, 6);
    match tuple {
        (first, ..) => {
            println!("First element is {}", first);
        }
    }

    match tuple {
        (first, .., last) => {
            println!("First element: {}, last element: {}", first, last);
        }
    }

    // 使用 @ 存储被匹配的值
    let p2 = Point { x: 1, y: 1 };
    match p2 {
        Point {
            x: val_x @ 1..=3,
            y: val_y @ 0..=11,
        } => {
            println!("val_x: {}, val_y: {}", val_x, val_y);
        }
        _ => {}
    }
}

struct Point {
    x: i32,
    y: i32,
}

// 使用 "_" 来忽略值
fn foo(_: i32, val: i32) {
    println!("val: {}", val);
}
