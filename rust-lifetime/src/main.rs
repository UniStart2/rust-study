use std::fmt::Display;

fn main() {
    let result;
    let string1 = String::from("1111");
    {
        let string2 = String::from("222");
        result = longest(string1.as_str(), string2.as_str());
        println!("{:?}", result);
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn longest2<'a, T>(x: &'a str, y: &'a str, z: T) -> &'a str
where
    T: Display,
{
    println!("{:}", z);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
