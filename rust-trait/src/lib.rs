mod string_parase {
    use regex::Regex;
    use std::str::FromStr;
    pub trait Parse {
        type Error;
        fn parse(s: &str) -> Result<Self, Self::Error>
        where
            Self: Sized;
    }

    impl<T> Parse for T
    where
        T: FromStr + Default,
    {
        // 定义关联类型 Error 为 String
        type Error = String;
        fn parse(s: &str) -> Result<Self, Self::Error> {
            let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
            if let Some(captures) = re.captures(s) {
                // 当出错时我们返回 Err(String)
                captures
                    .get(0)
                    .map_or(Err("failed to capture".to_string()), |s| {
                        s.as_str()
                            .parse()
                            .map_err(|_err| "failed to parse captured string".to_string())
                    })
            } else {
                Err("failed to parse string".to_string())
            }
        }
    }

    #[test]
    fn parse_should_work() {
        assert_eq!(u32::parse("123abcd"), Ok(123));
        assert_eq!(
            u32::parse("123.45abcd"),
            Err("failed to parse captured string".into())
        );
        assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
        assert!(f64::parse("abcd").is_err());
    }
}

mod add_override {
    use std::{ops::Add, process::Command};

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imagine: f64,
    }

    impl Complex {
        pub fn new(real: f64, imagine: f64) -> Self {
            Self { real, imagine }
        }
    }

    impl Add for Complex {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let real = self.real + rhs.real;
            let imagine = self.imagine + rhs.imagine;

            Self::new(real, imagine)
        }
    }

    impl Add for &Complex {
        // 注意返回值不应该是 Self 了，因为此时 Self 是 &Complex
        type Output = Complex;

        fn add(self, rhs: Self) -> Self::Output {
            let real = self.real + rhs.real;
            let imagine = self.imagine + rhs.imagine;

            Complex::new(real, imagine)
        }
    }

    #[test]
    fn test_complex() {
        let c1 = Complex::new(1f64, 1.1);
        let c2 = Complex::new(2 as f64, 1.2131);

        println!("{:?}", &c1 + &c2);

        println!("{:?}", c1 + c2);
    }
}

mod dyn_trait {

    pub trait Formatter {
        fn format(&self, input: &mut String) -> bool;
    }

    struct MarkdownFormatter;
    impl Formatter for MarkdownFormatter {
        fn format(&self, input: &mut String) -> bool {
            input.push_str("\nformatted with Markdown formatter");
            true
        }
    }

    struct RustFormatter;
    impl Formatter for RustFormatter {
        fn format(&self, input: &mut String) -> bool {
            input.push_str("\nformatted with Rust formatter");
            true
        }
    }

    struct HtmlFormatter;
    impl Formatter for HtmlFormatter {
        fn format(&self, input: &mut String) -> bool {
            input.push_str("\nformatted with HTML formatter");
            true
        }
    }

    pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
        for formatter in formatters {
            formatter.format(input);
        }
    }

    fn main() {
        let mut text = "Hello world!".to_string();
        let html: &dyn Formatter = &HtmlFormatter;
        let rust: &dyn Formatter = &RustFormatter;
        let formatters = vec![html, rust];
        format(&mut text, formatters);

        println!("text: {}", text);
    }
}
