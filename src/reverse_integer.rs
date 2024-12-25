#![allow(dead_code)]

pub fn solution(x: i32) -> i32 {
    let mut n = x.abs();
    let mut r = 0;
    const MAX: i32 = i32::MAX / 10;

    while n > 0 {
        if r > MAX {
            r = 0;
            break;
        }

        r *= 10;
        r += n % 10;
        n /= 10;
    }

    r * x.signum()
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test_reverse_integer() {
        assert!(solution(123) == 321);
        assert!(solution(-123) == -321);
        assert!(solution(120) == 21);
    }
}
