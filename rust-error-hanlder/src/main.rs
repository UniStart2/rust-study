mod error_handler {
    use std::{error::Error, fs::File, io::ErrorKind};

    pub fn test_error_handler() {
        let f = File::open("test.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("test.txt") {
                    Ok(new_file) => new_file,
                    Err(e) => panic!("Error creating file: {:?}", e),
                },
                oe => panic!("Error opening file: {:?}", oe),
            },
        };
    }

    pub fn test_unwrap() {
        let f = File::open("hello.txt").unwrap();
    }

    pub fn test_expect() {
        let f = File::open("hello.txt").expect("Not Found File");
    }
}

fn main() {
    error_handler::test_error_handler();

    // error_handler::test_unwrap();

    error_handler::test_expect();
}
