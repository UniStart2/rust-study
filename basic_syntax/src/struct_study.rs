// struct
pub struct User {
    pub name: String,
    pub email: String,
    pub sign_in_account: u64,
    pub active: bool,
}

impl User {
    pub fn build_user(name: String, email: String, sign_in_account: u64) -> User {
        User {
            name,
            email,
            sign_in_account,
            active: true,
        }
    }
}

// tuple struct
pub struct Color(pub i32, pub i32, pub i32);

// empty struct (unit-like struct)
#[warn(dead_code)]
pub struct Empty();

// 结构体中的方法
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub heigh: u32,
}

// Rectangle struct 的方法
impl Rectangle {
    // 实例方法
    pub fn area(&self) -> u32 {
        self.width * self.heigh
    }

    // 关联方法 -> 类似于其他语言中的类方法或者说静态方法
    pub fn square(edge: u32) -> Rectangle {
        Rectangle {
            width: edge,
            heigh: edge,
        }
    }
}
