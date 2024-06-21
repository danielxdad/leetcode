#![allow(dead_code)]

pub fn longest_palindrome_1(s: String) -> String {
    let mut max_pal_substr = String::new();

    for i in 0..s.len() {
        for j in (max_pal_substr.len().max(1))..=s.len() {
            if let Some(tmp) = s.get(i..i + j) {
                if tmp.chars().zip(tmp.chars().rev()).any(|(c1, c2)| c1 != c2) {
                    continue;
                }

                max_pal_substr = tmp.to_string();
            }
        }
    }

    max_pal_substr
}

pub fn longest_palindrome(s: String) -> String {
    let bytes: Vec<u8> = s.bytes().collect();
    let rev: Vec<u8> = std::iter::repeat(u8::MAX)
        .take(s.len().checked_sub(1).unwrap_or(0))
        .chain(bytes.iter().rev().map(|e| *e))
        .collect();
    let mut max_acc = (0, 0);

    /* println!(
        "{:?}\n{:?}",
        bytes.iter().collect::<Vec<_>>(),
        rev.iter().skip(s.len() - 1).collect::<Vec<_>>()
    );
    println!(
        "{}\n{}\n",
        String::from_iter(bytes.iter().map(|b| char::from(*b))),
        String::from_iter(rev.iter().skip(s.len() - 1).map(|b| char::from(*b)))
    );
    println!("{:?}", bytes.iter().zip(rev.iter().skip(s.len() - 1)).map(|(b1, b2)| b1 ^ b2).collect::<Vec<_>>()); */

    for i in 0..rev.len() {
        let mut start_offset: usize = usize::MAX;
        let mut end_offset: usize = usize::MAX;

        let acc = bytes
            .iter()
            .zip(rev.iter().skip(rev.len() - i))
            .map(|(b1, b2)| *b1 ^ *b2)
            .enumerate()
            .filter(|(_, e)| *e == 0)
            // .inspect(|(i, e)| println!("{}: {}", i, e))
            .fold((0, 0), |mut acc: (usize, usize), (i, _)| {
                if start_offset == usize::MAX {
                    start_offset = i;
                    end_offset = i;
                }

                if i - end_offset < 2 {
                    end_offset = i;
                } else {
                    start_offset = i;
                    end_offset = i + 1;
                }

                if (acc.1 - acc.0) < (end_offset - start_offset) {
                    acc = (start_offset, end_offset);
                }

                acc
            });

        if (max_acc.1 - max_acc.0) < (acc.1 - acc.0) {
            max_acc = acc;
        }
    }

    s.get(max_acc.0..=max_acc.1)
        .and_then(|slice| Some(String::from(slice)))
        .unwrap_or(String::new())
}

#[cfg(test)]
mod test {
    use super::longest_palindrome_1;

    #[test]
    fn test_last_remaining() {
        let strs = vec![
            ("", ""),
            ("a", "a"),
            ("ac", "c"),
            ("babad", "aba"),
            ("cbbd", "bb"),
            ("abb", "bb"),
            ("aacabdkacaa", "aca"),
            ("bb", "bb"),
        ];
        for (input, output) in strs {
            assert!(longest_palindrome_1(input.to_string()) == output.to_string());
        }
    }
}
