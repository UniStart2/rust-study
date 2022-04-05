use crate::trait_test::notify1;

mod generic_test {

    #[derive(Debug)]
    pub struct Response<T> {
        pub data: T,
    }

    impl<T: std::fmt::Debug> Response<T> {
        pub fn print(param: T) {
            println!("{:?}", param)
        }

        pub fn print_self(&self) {
            println!("{:?}", &self)
        }
    }
}

mod trait_test {
    use core::fmt::Formatter;
    use std::fmt::Display;
    pub trait Summary {
        fn summary_fn(&self) -> String;
    }

    pub struct A {
        pub data: String,
    }

    impl Summary for A {
        fn summary_fn(&self) -> String {
            format!("{} - {}", &self, "impl Summary trait")
        }
    }

    impl std::fmt::Display for A {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            f.write_fmt(format_args!("impl Display for {}", self.data))
        }
    }

    impl A {
        pub fn print(&self, T: impl std::fmt::Display) {
            format!("{} - {}", &self, T);
        }
    }

    pub fn notify1<T: Summary + Display, U: Clone + Display>(a: T, b: U) -> String {
        format!("Breaking news {}", a.summary_fn())
    }

    // 等价写法
    pub fn notify2<T, U>(a: T, b: U) -> String
    where
        T: Summary + Display,
        U: Clone + Display,
    {
        format!("Breaking news {}", a.summary_fn())
    }
}

fn main() {
    use generic_test::Response;

    let res1 = Response { data: 1 };
    let res2 = Response { data: "str" };
    let res3 = Response { data: 13.13 };
    let res4 = Response { data: () };
    let res5 = Response { data: (1, 2, 3) };
    let res6 = Response {
        data: Response { data: {} },
    };

    Response::print("1");
    Response::print(1);
    Response::print(123.213);

    res1.print_self();
    res2.print_self();
    res3.print_self();
    res4.print_self();
    res5.print_self();
    res6.print_self();

    use trait_test::notify2;
    use trait_test::A;
    let t1 = A {
        data: "struct A".to_string(),
    };
    let t2 = A {
        data: "struct A2".to_string(),
    };
    println!("{:}", t1);

    println!("{}", notify1(t1, 2.111));
    println!("{}", notify2(t2, "dsads".to_string()));
}
