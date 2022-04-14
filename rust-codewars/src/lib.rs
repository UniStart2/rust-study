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

///
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
