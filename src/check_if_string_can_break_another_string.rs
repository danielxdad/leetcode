// https://leetcode.com/problems/check-if-a-string-can-break-another-string/description/
#![allow(dead_code)]

pub fn solution(s1: String, s2: String) -> bool {
    let mut s1_vec = Vec::from(s1.as_str());
    let mut s2_vec = Vec::from(s2.as_str());
    let mut s1_small = true;
    let mut s2_small = true;

    s1_vec.sort_unstable();
    s2_vec.sort_unstable();

    for i in 0..s1_vec.len() {
        let cs1 = s1_vec[i];
        let cs2 = s2_vec[i];

        s1_small &= cs1 <= cs2;
        s2_small &= cs2 <= cs1;
    }

    return s1_small || s2_small;
}
