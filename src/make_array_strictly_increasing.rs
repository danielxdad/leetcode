// https://leetcode.com/problems/make-array-strictly-increasing/
#![allow(dead_code)]

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

    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);
    println!("{:-<30}\n", "");

    let mut mask: Vec<_> = arr1
        .windows(2)
        .enumerate()
        .map(|(index, slice )| (slice[0] < slice[1], index))
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

        if let Some(&n) = arr2.iter().rev().find(|&&n| arr1[lower_index] < n && n < arr1[upper_index]) {
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
            .map(|(index, slice )| (slice[0] < slice[1], index))
            .collect();
    }

    ops
}

pub fn make_array_increasing_v3(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    if arr1.len() < 2 {
        return 0;
    }
    
    let original_arr1 = &arr1;
    let mut arr1 = arr1.clone();
    let mut arr2: Vec<i32> = arr2.iter().collect::<HashSet<_>>().iter().map(|&&n| n).collect();

    arr2.sort_unstable();

    dbg!(format!("{:?}", arr1));
    dbg!(format!("{:?}", arr2));
    println!("{:-<150}\n", "");

    fn compute_boolean_mask (arr1: &Vec<i32>) -> Vec<bool> {
        let mut mask = vec![false; arr1.len()];

        mask.first_mut().map(|e| *e = arr1.iter().skip(1).all(|&n| arr1[0] < n));

        for i in 1..arr1.len() - 1 {
            mask[i] = arr1[i - 1] < arr1[i] && arr1[i] < arr1[i + 1];
        }

        mask.last_mut().map(|e| *e = arr1.iter().rev().skip(1).all(|&n| n < arr1[arr1.len() - 1]));

        mask
    }

    let mut mask = compute_boolean_mask(&arr1);

    if mask[0] == false {
        arr1[0] = arr2[0];
    }

    if mask[mask.len() - 1] == false {
        arr1.last_mut().map(|e| *e = arr2[arr2.len() - 1]);
    }

    dbg!(format!("{:?}", mask));
    dbg!(format!("{:?}", arr1));
    println!("{:-<150}\n", "");

    for i in 1..arr1.len() - 1 {
        if let Some(&n) = arr2.iter().find(|&&n| n > arr1[i - 1]) {
            arr1[i] = n;

            dbg!(format!("{:?}", mask));
            dbg!(format!("{:?}", arr1));
            println!("{:-<150}\n", "");

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

    arr1
        .iter()
        .zip(original_arr1)
        .map(|(&n1, &n2)| (n1 != n2) as i32 )
        // .inspect(|e| println!("{}", e))
        .fold(0, |acc, e| acc + e)
}

#[cfg(test)]
mod test {
    use super::{
        /* make_array_increasing_v2 ,*/
        make_array_increasing_v3
    };

    #[test]
    fn test_make_array_increasing() {
        // println!("{}", make_array_increasing_v2([1,5,3,6,7].to_vec(), [1,3,2,4].to_vec()));
        // println!("{}", make_array_increasing_v2([0,11,6,1,4,3].to_vec(), [5,4,11,10,1,0].to_vec()));

        // println!("{}", make_array_increasing_v3([0,11,6,1,4,3].to_vec(), [5,4,11,10,1,0].to_vec()));
        // println!("{}", make_array_increasing_v3([1,5,3,6,7].to_vec(), [1,3,2,4].to_vec()));
        // println!("{}", make_array_increasing_v3([1,5,3,6,7].to_vec(), [1,6,3,3].to_vec()));

        // [5,16,19,2,1,12,7,14,5,16]
        // [6,17,4,3,6,13,4,3,18,17,16,7,14,1,16]
        /* 
         * arr1.first() < arr1.last() & 
         * arr1.last() - arr1.first() >= |uniq( n in arr2 -> n > arr1.first() && n < arr1.last() )|
         */
        println!("{}",
            make_array_increasing_v3(
                [5,16,19,2,1,12,7,14,5,16].to_vec(),
                [6,17,4,3,6,13,4,3,18,17,16,7,14,1,16].to_vec()
            )
        );
        assert!(1 == 2);

        assert!(make_array_increasing_v3([1,5,3,6,7].to_vec(), [1,3,2,4].to_vec()) == 1);
        assert!(make_array_increasing_v3([1,5,3,6,7].to_vec(), [4,3,1].to_vec()) == 2);
        assert!(make_array_increasing_v3([1,5,3,6,7].to_vec(), [1,6,3,3].to_vec()) == -1);
        assert!(make_array_increasing_v3([7,6,3,0,3].to_vec(), [9].to_vec()) == -1);
        assert!(make_array_increasing_v3([0,11,6,1,4,3].to_vec(), [5,4,11,10,1,0].to_vec()) == 5);

        // Replace 5 with 2, then arr1 = [1, 2, 3, 6, 7]
        // assert!(make_array_increasing_rec([1,5,3,6,7].to_vec(), [1,3,2,4].to_vec()) == 1);
        // Replace 5 with 3 and then replace 3 with 4. arr1 = [1, 3, 4, 6, 7].
        // println!("{}", make_array_increasing_rec([1,5,3,6,7].to_vec(), [4,3,1].to_vec()));
        // assert!(make_array_increasing_rec([1,5,3,6,7].to_vec(), [4,3,1].to_vec()) == 2);
        // You can't make arr1 strictly increasing.
        // assert!(make_array_increasing_rec([1,5,3,6,7].to_vec(), [1,6,3,3].to_vec()) == -1);
    }
}
