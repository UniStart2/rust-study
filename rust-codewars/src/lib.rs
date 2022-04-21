/// Credit Card Mask
/// Return a String with all characters masked as '#' except the last 4.
mod credit_card_mask {

    fn maskify(cc: &str) -> String {
        // let len = cc.len();

        // if len <= 4 {
        //     return cc.to_string();
        // }

        // let last4ch = &cc[len - 4..len];
        // let mut mask_str = String::with_capacity(len - 4);
        // for i in 0..len - 4 {
        //     mask_str.insert(i, '#');
        // }

        // mask_str + last4ch

        match cc.len() > 4 {
            true => "#".repeat(cc.len() - 4) + &cc[cc.len() - 4..],
            false => cc.to_string(),
        }
    }

    #[cfg(test)]
    mod tests {

        use super::maskify;

        #[test]
        fn it_masks_example_strings() {
            assert_eq!(maskify("4556364607935616"), "############5616");
            assert_eq!(maskify("1"), "1");
            assert_eq!(maskify("11111"), "#1111");
        }
    }
}

/// find_next_square
mod next_square {

    fn find_next_square(sq: u64) -> Option<u64> {
        let s = (sq as f64).sqrt();

        if s.fract() != 0f64 {
            return None;
        }

        Some((s as u64 + 1).pow(2))
    }

    #[cfg(test)]
    mod tests {
        use super::find_next_square;

        #[test]
        fn sample_tests() {
            assert_eq!(find_next_square(121), Some(144));
            assert_eq!(find_next_square(625), Some(676));
            assert_eq!(find_next_square(319_225), Some(320_356));
            assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
            assert_eq!(find_next_square(155), None);
            assert_eq!(find_next_square(342_786_627), None);
        }
    }
}

/// Bouncing Balls
/// A child is playing with a ball on the nth floor of a tall building.
///  The height of this floor, h, is known.
/// He drops the ball out of the window.
/// The ball bounces (for example), to two-thirds of its height (a bounce of 0.66).
/// His mother looks out of a window 1.5 meters from the ground.
/// How many times will the mother see the ball pass in front of her window
/// (including when it's falling and bouncing?
///
/// Three conditions must be met for a valid experiment:
///  Float parameter "h" in meters must be greater than 0
///  Float parameter "bounce" must be greater than 0 and less than 1
///  Float parameter "window" must be less than h.
/// If all three conditions above are fulfilled, return a positive integer,
///  otherwise return -1.
///
/// Note:
/// The ball can only be seen if the height of the rebounding ball is strictly greater than the window parameter.
mod bounce_ball {
    fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
        if h > 0f64 && bounce > 0f64 && bounce < 1f64 && window < h {
            let mut tmp = h;
            let mut count = 0; // 统计小球可以反弹次数
            loop {
                tmp *= bounce;

                if tmp <= window {
                    break;
                }
                count += 1;
            }

            return count * 2 + 1;
        }

        -1
    }

    #[test]
    fn test() {
        println!("{}", bouncing_ball(1.0, 0.6, 0.6));
    }

    fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
        assert_eq!(bouncing_ball(h, bounce, window), exp)
    }

    #[test]
    fn tests_bouncing_ball() {
        testequal(3.0, 0.66, 1.5, 3);
        testequal(30.0, 0.66, 1.5, 15);
        testequal(40.0, 0.4, 10.0, 3);
        testequal(10.0, 0.6, 10.0, -1);
        testequal(3.0, 0.1, 1.5, 1);
    }
}

/// Find Multiples of a Number
mod find_multiples_of_a_number {
    fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
        // let mut vec: Vec<u32> = vec![n];
        // let mut i = 2;
        // while n * i <= limit {
        //     vec.insert((i - 1) as usize, n * i);
        //     i = i + 1;
        // }
        // vec

        (n..limit).step_by((n) as usize).collect()
    }

    #[test]
    fn basic_test() {
        assert_eq!(find_multiples(1, 2), [1, 2]);
        assert_eq!(find_multiples(5, 7), [5]);
        assert_eq!(find_multiples(4, 27), [4, 8, 12, 16, 20, 24]);
        assert_eq!(find_multiples(11, 54), [11, 22, 33, 44]);
        assert_eq!(find_multiples(5, 25), [5, 10, 15, 20, 25]);
    }
}

/// Categorize New Member
mod categorize_new_member {
    use std::collections::btree_map::IterMut;

    fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
        let mut vec = vec![];

        for i in 0..data.len() {
            let (age, handicap) = data.get(i).unwrap();
            if *age >= 55 && *handicap > 7 {
                vec.insert(i, "Senior".to_string());
            } else {
                vec.insert(i, "Open".to_string());
            }
        }

        vec
    }

    #[test]
    fn returns_expected() {
        assert_eq!(
            open_or_senior(vec![(45, 12), (55, 21), (19, -2), (104, 20)]),
            vec!["Open", "Senior", "Open", "Senior"]
        );
        assert_eq!(
            open_or_senior(vec![(3, 12), (55, 1), (91, -2), (54, 23)]),
            vec!["Open", "Open", "Open", "Open"]
        );
    }
}

mod cubes_number {
    use regex::Regex;

    fn is_sum_of_cubes(s: &str) -> String {
        let is_cubes_number = |n: &u64| -> bool {
            let mut num = *n;
            let mut sum = 0;

            while (num > 0) {
                sum += (num % 10).pow(3);
                num /= 10;
            }

            sum == *n
        };

        let digital_cubes = Regex::new(r"\d{1,3}")
            .unwrap()
            .find_iter(s)
            .filter_map(|m| m.as_str().parse::<u64>().ok().filter(is_cubes_number))
            .collect::<Vec<_>>();

        if digital_cubes.is_empty() {
            return String::from("Unlucky");
        }

        format!(
            "{} {} Lucky",
            digital_cubes
                .iter()
                .map(|iter| { iter.to_string() })
                .collect::<Vec<_>>()
                .join(" "),
            digital_cubes.iter().sum::<u64>(),
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn dotest(s: &str, exp: &str) -> () {
            println!("s: {:?};", s);
            let ans = is_sum_of_cubes(s);
            println!("actual:\n{:?};", ans);
            println!("expect:\n{:?};", exp);
            println!("{};", ans == exp);
            assert_eq!(ans, exp);
            println!("{};", "-");
        }

        #[test]
        fn basic_tests() {
            dotest("00 9026315 -827&()", "0 0 Lucky");
            dotest("0 9026315 -827&()", "0 0 Lucky");
            dotest(
                "Once upon a midnight dreary, while100 I pondered, 9026315weak and weary -827&()",
                "Unlucky",
            );
        }
    }
}
