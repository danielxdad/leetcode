#![allow(dead_code)]
use std::collections::HashMap;

fn my_atoi(s: String) -> i32 {
    let base = 10;
    let mut sign: Option<i32> = None;
    let mut r: i32 = 0;

    for ch in s.trim().chars() {
        if sign.is_none() {
            (sign, r) = match ch {
                '+' => (Some(1), 0),
                '-' => (Some(-1), 0),
                _ if ch.is_digit(base) => (Some(1), ch.to_digit(base).unwrap() as i32),
                _ => (None, 0),
            };

            if sign.is_none() {
                break;
            }

            continue;
        }

        if let Some(d) = ch.to_digit(base) {
            r = r.saturating_mul(10);
            if sign.unwrap() == 1 {
                r = r.saturating_add(d as i32);
            } else {
                r = r.saturating_sub(d as i32);
            }
        } else {
            break;
        }
    }

    r
}

pub fn my_atoi_hashmap(s: String) -> i32 {
    let base = 10;
    let hm: HashMap<char, i32> =
        HashMap::from_iter((0..=9).map(|n| (char::from_digit(n as u32, base).unwrap(), n)));
    let mut sign = None;
    let mut sign_index = 0;
    let mut r = 0;

    for (i, ch) in s.chars().enumerate() {
        if sign.is_none() {
            if let Some(d) = hm.get(&ch) {
                (sign, r) = (Some(1), *d);
                sign_index = i;
                break;
            }

            if ch == '+' {
                (sign, r) = (Some(1), 0);
                sign_index = i;
                break;
            } else if ch == '-' {
                (sign, r) = (Some(-1), 0);
                sign_index = i;
                break;
            } else if ch.is_whitespace() {
                continue;
            } else {
                return r;
            }
        } else {
            break;
        }
    }

    // println!("{:?}, {}", sign, sign_index);
    // println!("{}\n-----------\n", r);
    // println!("{:?}", hm);

    for ch in s.chars().skip(sign_index + 1) {
        if let Some(d) = hm.get(&ch) {
            // println!("d: {}", d);
            r = r
                .saturating_mul(10)
                .saturating_add((*d as i32) * sign.unwrap());
        } else {
            break;
        }
    }

    r
}

#[cfg(test)]
mod test {
    use super::{my_atoi, my_atoi_hashmap};

    #[test]
    fn test_string_to_integer_atoi() {
        assert!(my_atoi(" -042".to_string()) == -42);
        assert!(my_atoi("42".to_string()) == 42);
        assert!(my_atoi("words and 987".to_string()) == 0);
        assert!(my_atoi("-91283472332".to_string()) == -2147483648);

        assert!(my_atoi_hashmap(" -042".to_string()) == -42);
        assert!(my_atoi_hashmap("42".to_string()) == 42);
        assert!(my_atoi_hashmap("words and 987".to_string()) == 0);
        assert!(my_atoi_hashmap("-91283472332".to_string()) == -2147483648);
    }
}
