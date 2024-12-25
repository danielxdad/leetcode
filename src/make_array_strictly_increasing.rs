#![allow(dead_code, unused_imports, unused_mut)]
// https://leetcode.com/problems/make-array-strictly-increasing/

use std::collections::HashSet;

pub fn make_array_increasing_v2(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    // TODO: iterar por arr1 en ventanas de 2 elementos de forma descendente
    // y ascendente intercaladamente si es descendente -> arr1[n - 1] < arr1[n] (1)
    // si es ascendente -> arr1[n] < arr1[n + 1] (2)
    // si no se cumple (1), entonces buscar/remplazar arr1[n - 1] con un numero en arr2 <= arr1[n]
    // si no se cumple (2), entonces  buscar/remplazar arr1[n + 1 con un numero en arr2 => arr1[n]
    if arr1.len() < 2 {
        return 0;
    }

    let mut arr1 = arr1.clone();
    let mut arr2 = arr2.clone();
    let mut ops = 0;
    let mut _i = 0;

    arr2.sort_unstable();
    arr2.dedup();

    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);
    println!("{:-<30}\n", "");

    let mut mask: Vec<_> = arr1
        .windows(2)
        .enumerate()
        .map(|(index, slice)| (slice[0] < slice[1], index))
        .collect();

    while mask.iter().any(|(ok, _)| *ok == false) {
        let mut lower_index: usize = 0;
        let mut upper_index: usize = mask.len() - 1;

        eprintln!("{:?}", mask);

        for e in mask.iter() {
            if e.0 == false {
                break;
            }
            lower_index = e.1;
        }

        for e in mask.iter().skip(lower_index + 1) {
            if e.0 {
                upper_index = e.1;
                break;
            }
        }

        println!("{} - {}", lower_index, upper_index);

        if let Some(&n) = arr2
            .iter()
            .rev()
            .find(|&&n| arr1[lower_index] < n && n < arr1[upper_index])
        {
            arr1[upper_index - 1] = n;
            ops += 1;
        } else {
            let current_num = arr1[upper_index];

            upper_index = usize::min(upper_index + 1, arr1.len() - 1);

            if let Some(&n) = arr2
                .iter()
                .rev()
                .find(|&&n| arr1[lower_index] < n && n < arr1[upper_index] && n != current_num)
            {
                arr1[upper_index - 1] = n;
                ops += 1;
            } else {
                ops = -1;
                break;
            }
        }

        println!("{:?}", arr1);

        mask = arr1
            .windows(2)
            .enumerate()
            .map(|(index, slice)| (slice[0] < slice[1], index))
            .collect();
    }

    ops
}

pub fn make_array_increasing_v3(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    if arr1.len() < 2 {
        return 0;
    }

    let original_arr1: &Vec<i32> = &arr1;
    let mut arr1: Vec<i32> = arr1.clone();
    let mut arr2: Vec<i32> = arr2
        .iter()
        .collect::<HashSet<_>>()
        .iter()
        .map(|&&n| n)
        .collect();

    arr2.sort_unstable();
    arr2.dedup();

    dbg!(format!("{:?}", arr1));
    dbg!(format!("{:?}", arr2));
    println!("{:-<150}\n", "");

    fn compute_boolean_mask(arr1: &Vec<i32>) -> Vec<bool> {
        let mut mask = vec![false; arr1.len()];

        mask.first_mut()
            .map(|e| *e = arr1.iter().skip(1).all(|&n| arr1[0] < n));

        for i in 1..arr1.len() - 1 {
            mask[i] = arr1[i - 1] < arr1[i] && arr1[i] < arr1[i + 1];
        }

        mask.last_mut()
            .map(|e| *e = arr1.iter().rev().skip(1).all(|&n| n < arr1[arr1.len() - 1]));

        mask
    }

    let mut mask = compute_boolean_mask(&arr1);

    if mask[0] == false {
        arr1[0] = arr2[0];
        // mask[0] = true;
    }

    if mask[mask.len() - 1] == false {
        arr1.last_mut().map(|e| *e = arr2[arr2.len() - 1]);
        // mask.last_mut().map(|e| *e = true);
    }

    dbg!(format!("{:?}", mask));
    dbg!(format!("{:?}", arr1));
    println!("{:-<150}\n", "");

    for i in 1..arr1.len() - 1 {
        if let Some(p) = arr2.iter().position(|&n| n > arr1[i - 1]) {
            dbg!(format!("{:?}", mask));
            // dbg!(format!("{:?}", arr1));
            // dbg!(format!("i: {}; {} -> {}", i, arr1[i], arr2[p]));

            arr1[i] = arr2[p];
            // arr2.remove(p);

            dbg!(format!("{:?}", arr1));
            dbg!(format!("{:?}", arr2));
            println!("{:-<150}\n", "");

            // mask[i - 1] = arr1[i - 1] < arr1[i] && arr1[i] < arr1[i + 1];
            mask = compute_boolean_mask(&arr1);
            if mask.iter().all(|&e| e) {
                break;
            }
        }
    }

    dbg!(format!("{:?}", mask));
    dbg!(format!("{:?}", arr1));
    println!("{:-<150}\n", "");

    if mask.iter().any(|&e| e == false) {
        return -1;
    }

    dbg!(format!("{:?}", original_arr1));
    dbg!(format!("{:?}", arr2));

    arr1.iter()
        .zip(original_arr1)
        .map(|(&n1, &n2)| (n1 != n2) as i32)
        // .inspect(|e| println!("{}", e))
        .fold(0, |acc, e| acc + e)
}

pub fn make_array_increasing_v4(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    if arr1.len() < 2 {
        return 0;
    }

    let original_arr1 = &arr1;
    let arr1 = arr1.clone();
    let mut arr2: Vec<i32> = arr2
        .iter()
        .collect::<HashSet<_>>()
        .iter()
        .map(|&&n| n)
        .collect();

    arr2.sort_unstable();

    dbg!(format!("{:?}", arr2));

    fn recursive(arr1: Vec<i32>, arr2: &Vec<i32>) -> Result<Vec<i32>, ()> {
        dbg!(format!("{:?}", arr1));
        println!("{:-<150}\n", "");

        if arr1.len() < 2 {
            return Ok(arr1);
        }

        let middle: usize = arr1.len() / 2;
        let mut left = recursive(arr1[0..middle].to_vec(), arr2)?;
        let mut right = recursive(arr1[middle..].to_vec(), arr2)?;

        if left[left.len() - 1] >= right[0] {
            let lower: i32 = left[left.len() - 1];
            let upper = if right.len() > 1 { right[1] } else { right[0] };

            dbg!(format!("left: {:?}; right: {:?}", left, right));
            dbg!(format!("lower: {}; upper: {}", lower, upper));

            if let Some(&n) = arr2.iter().find(|&&n| lower < n && n < upper) {
                dbg!(format!("found replacement: {}", n));
                right.first_mut().map(|e| *e = n);
            } else {
                let lower = if left.len() > 1 {
                    left[left.len() - 2]
                } else {
                    left[left.len() - 1]
                };
                let upper = right[0];

                dbg!(format!("lower: {}; upper: {}", lower, upper));

                if let Some(&n) = arr2.iter().find(|&&n| lower < n && n < upper) {
                    dbg!(format!("found replacement: {}", n));
                    left.last_mut().map(|e| *e = n);
                } else {
                    return Err(());
                }
            }
        }

        left.extend(right);

        return Ok(left);
    }

    if let Ok(result) = recursive(arr1, &arr2) {
        return result
            .iter()
            .zip(original_arr1)
            .map(|(&n1, &n2)| (n1 != n2) as i32)
            .fold(0, |acc, e| acc + e);
    }

    -1
}

pub fn make_array_increasing_v5(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    if arr1.len() < 2 {
        return 0;
    }

    let mut arr1: Vec<i32> = arr1.clone();
    let mut arr2: Vec<i32> = arr2
        .iter()
        .collect::<HashSet<_>>()
        .iter()
        .map(|&&n| n)
        .collect();

    arr2.sort_unstable();

    dbg!(format!("{:?}", arr1));
    dbg!(format!("{:?}", arr2));
    println!("{:-<100}\n", "");

    fn recursive(arr1: Vec<i32>, arr2: &Vec<i32>) -> i32 {
        if arr1.len() < 2 {
            return 0;
        }

        let middle: usize = arr1.len() / 2;
        let arr2_median = (arr2[0] + arr2[arr2.len() - 1]) as f32 / 2.0;
        let mut pivot = i32::MAX;

        println!("{}", arr2_median);

        for &i in &arr2[(arr2.len() / 2 - 1)..=(arr2.len() / 2 + 1)] {
            let diff = (i as f32 - arr2_median).abs();
            let previous_diff = (pivot as f32 - arr2_median).abs();
            println!("{}, {}, {}", i, diff, previous_diff);
            if diff < previous_diff {
                pivot = i;
            }
        }

        // if pivot != arr1[middle] {
        //     arr1[middle] = pivot;
        // }

        println!("pivot: {}", pivot);

        let left = recursive(arr1[0..middle].to_vec(), arr2);
        let right = recursive(arr1[(middle + 1)..].to_vec(), arr2);

        left + ((pivot != arr1[middle]) as i32) + right
    }

    recursive(arr1, &arr2)
}

#[cfg(test)]
mod test {
    use super::{make_array_increasing_v3, make_array_increasing_v4};

    #[test]
    fn test_make_array_increasing() {
        // println!("{}", make_array_increasing_v2([1,5,3,6,7].to_vec(), [1,3,2,4].to_vec()));
        // println!("{}", make_array_increasing_v2([0,11,6,1,4,3].to_vec(), [5,4,11,10,1,0].to_vec()));

        // dbg!(make_array_increasing_v3(
        //     [0, 11, 6, 1, 4, 3].to_vec(),
        //     [5, 4, 11, 10, 1, 0].to_vec()
        // ));
        // dbg!(make_array_increasing_v3(
        //     [1, 5, 3, 6, 7].to_vec(),
        //     [1, 3, 2, 4].to_vec()
        // ));
        // dbg!(make_array_increasing_v3(
        //     [1, 5, 3, 6, 7].to_vec(),
        //     [1, 6, 3, 3].to_vec()
        // ));

        // [5,16,19,2,1,12,7,14,5,16]
        // [6,17,4,3,6,13,4,3,18,17,16,7,14,1,16]
        /*
         * arr1.first() < arr1.last() &
         * arr1.last() - arr1.first() >= |uniq( n in arr2 -> n > arr1.first() && n < arr1.last() )|
         *
         * middle point in arr1 -> root of tree
         */

        // println!("{}",
        //     make_array_increasing_v4(
        //         [5, 16, 19, 2, 1, 12, 7, 14, 5, 16].to_vec(),
        //         [6, 17, 4, 3, 6, 13, 4, 3, 18, 17, 16, 7, 14, 1, 16].to_vec()
        //     )
        // );
        // assert!(1 == 2);

        // assert!(make_array_increasing_v4([1,5,3,6,7].to_vec(), [1,3,2,4].to_vec()) == 1);
        // assert!(make_array_increasing_v4([1, 5, 3, 6, 7].to_vec(), [4, 3, 1].to_vec()) == 2);
        // assert!(make_array_increasing_v4([1,5,3,6,7].to_vec(), [1,6,3,3].to_vec()) == -1);
        // assert!(make_array_increasing_v4([7,6,3,0,3].to_vec(), [9].to_vec()) == -1);
        // assert!(make_array_increasing_v4([0,11,6,1,4,3].to_vec(), [5,4,11,10,1,0].to_vec()) == 5);
    }
}
