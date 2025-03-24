/*
给你一个字符串数组 words 和一个字符串 s ，其中 words[i] 和 s 只包含 小写英文字母 。

请你返回 words 中是字符串 s 前缀 的 字符串数目 。

一个字符串的 前缀 是出现在字符串开头的子字符串。子字符串 是一个字符串中的连续一段字符序列。

输入：words = ["a","b","c","ab","bc","abc"], s = "abc"
输出：3
解释：
words 中是 s = "abc" 前缀的字符串为：
"a" ，"ab" 和 "abc" 。
所以 words 中是字符串 s 前缀的字符串数目为 3 。

输入：words = ["a","a"], s = "aa"
输出：2
解释：
两个字符串都是 s 的前缀。
注意，相同的字符串可能在 words 中出现多次，它们应该被计数多次。

*/

struct Solution;

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut cnt = 0;
        for word in words {
            if s.starts_with(&word) {
                cnt += 1;
            }
        }
        cnt
    }

    pub fn count_prefixes_iter(words: Vec<String>, s: String) -> i32 {
        words
            .into_iter()
            .filter(|w| s.starts_with(w.as_str()))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::p2256_count_prefixes::Solution;

    type Fn = fn(words: Vec<String>, s: String) -> i32;

    fn run_test_case(func: Fn, words: &[&str], s: String, expected: i32) {
        let words = words.iter().map(|s| String::from(*s)).collect();
        assert_eq!(expected, func(words, s));
    }

    #[test]
    fn test_two_sum_methods() {
        let methods: Vec<Fn> = vec![Solution::count_prefixes, Solution::count_prefixes_iter];

        for method in methods {
            run_test_case(
                method,
                &["a", "b", "c", "ab", "bc", "abc"],
                String::from("abc"),
                3,
            );
            run_test_case(method, &["a", "a"], String::from("aa"), 2);
            run_test_case(method, &["a", "a"], String::from("mbc"), 0);
            run_test_case(
                method,
                &[
                    "feh", "w", "w", "lwd", "c", "s", "vk", "zwlv", "n", "w", "sw", "qrd", "w",
                    "w", "mqe", "w", "w", "w", "gb", "w", "qy", "xs", "br", "w", "rypg", "wh", "g",
                    "w", "w", "fh", "w", "w", "sccy",
                ],
                String::from("w"),
                14,
            );
        }
    }
}
