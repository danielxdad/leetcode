#[allow(dead_code)]
// https://leetcode.com/problems/elimination-game/

pub fn last_remaining(n: i32) -> i32 {
    /* let mut v: Vec<i32> = (1..=n).skip(1).step_by(2).collect();
    let mut ascending = false;
    if n < 2 {
        return  n;
    }
    while v.len() > 1 {
        if ascending {
            v = v.iter().skip(1).step_by(2).map(|e| *e).collect();
        } else {
            v = v.iter().rev().skip(1).step_by(2).rev().map(|e| *e).collect();
        }
        ascending = !ascending;
    }
    v[0] */
    // ==========================================================================

    let mut start = 1;
    let mut end = n;
    let mut step = 1;
    let mut asc = true;
    let mut el = n;
    
    while el > 1 {
        if asc {
            start += step;
            end -= step * (el % 2);
        } else {
            end -= step;
            start += step * (el % 2);
        }

        asc = !asc;
        step *= 2;
        el = (end - start) / step + 1;
    }

    start
}

#[cfg(test)]
mod test {
    use super::last_remaining;

    #[test]
    fn test_last_remaining() {
        // println!("{}", last_remaining(2));
        
        assert!(last_remaining(1) == 1);
        assert!(last_remaining(2) == 2);
        assert!(last_remaining(3) == 2);
        assert!(last_remaining(4) == 2);
        assert!(last_remaining(5) == 2);
        assert!(last_remaining(6) == 4);
        assert!(last_remaining(7) == 4);
        assert!(last_remaining(8) == 6);
        assert!(last_remaining(9) == 6);
        assert!(last_remaining(10) == 8);
        assert!(last_remaining(11) == 8);
        assert!(last_remaining(12) == 6);
        assert!(last_remaining(13) == 6);
        assert!(last_remaining(14) == 8);
        assert!(last_remaining(15) == 8);
        assert!(last_remaining(16) == 6);
        assert!(last_remaining(17) == 6);
    }
}
