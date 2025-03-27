/*
    给定一个字符串 s ，请你找出其中不含有重复字符的 最长 子串 的长度。
*/
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut set = HashSet::new();
        let mut left = 0;
        let mut max_len = 0;

        for (right, &b) in bytes.iter().enumerate() {
            while !set.insert(b) {
                set.remove(&bytes[left]);
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::p0003_length_of_longest_substring::Solution;

    type Fn = fn(s: String) -> i32;

    fn run_test_case(func: Fn, str: String, expected: i32) {
        assert_eq!(expected, func(str));
    }

    #[test]
    fn test_two_sum_methods() {
        let methods: Vec<Fn> = vec![Solution::length_of_longest_substring];

        for method in methods {
            run_test_case(method, String::from("abcabcbb"), 3);
            run_test_case(method, String::from("bbbbb"), 1);
            run_test_case(method, String::from("pwwkew"), 3);
            run_test_case(method, String::from("cn"), 2);
            run_test_case(method, String::from("dvdf"), 3);
        }
    }
}
