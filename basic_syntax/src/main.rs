mod enum_study;
mod struct_study;

use enum_study::Message;
use struct_study::*;

fn main() {
    let name = String::from("Mike");
    let email = String::from("sdadas@gmail.com");
    let user_one = User::build_user(name, email, 12312u64);
    println!("user_one name: {}", user_one.email);

    let user_two = User {
        name: String::from("dsadas"),
        email: String::from("sdadaqeqw@qq.com"),
        sign_in_account: 23123u64,
        ..user_one
    };

    println!("user_two name: {}", user_two.active);

    let color = Color(0, 255, 127);
    println!("[0]:{:?}, [1]:{:?}, [2]:{:?}", color.0, color.1, color.2);

    let rectangle = Rectangle {
        width: 32,
        heigh: 64,
    };
    println!("rectangle area: {:?}", rectangle.area());

    let square = Rectangle::square(10);
    println!("square area: {:?}", square);

    let content: String = String::from("This is an message!!!");
    let msg: Message = Message::new(4821312u32, 2312312u32, content);
    println!("msg: {:?}", msg);

    let op1 = Some(5);
    let op2: Option<i32> = None;
    let basicNum = 1;
}
