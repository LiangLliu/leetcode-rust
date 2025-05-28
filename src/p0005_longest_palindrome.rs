/*
给你一个字符串 s，找到 s 中最长的 回文 子串。
回文性： 如果字符串向前和向后读都相同，则它满足 回文性。

示例 1：

输入：s = "babad"
输出："bab"
解释："aba" 同样是符合题意的答案。
示例 2：

输入：s = "cbbd"
输出："bb"


提示：

1 <= s.length <= 1000
s 仅由数字和英文字母组成

 */

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let sv = s.chars().collect::<Vec<char>>();
        let mut window = s.len();
        let mut head = 0;
        while head != sv.len() {
            if head + window > sv.len() {
                window -= 1;
                head = 0;
                continue;
            }
            if Self::is_palindrome(&sv[head..head + window]) {
                return sv[head..head + window].iter().collect::<String>();
            }
            head += 1;
        }

        return "".to_string();
    }
    fn is_palindrome(v: &[char]) -> bool {
        for i in 0..v.len() / 2 {
            if v[i] != v[v.len() - 1 - i] {
                return false;
            }
        }
        true
    }

    pub fn longest_palindrome2(s: String) -> String {
        let mut t = Vec::with_capacity((s.len() << 1) + 3);
        t.push('^');
        s.chars().for_each(|x| {
            t.push('#');
            t.push(x);
        });
        t.push('#');
        t.push('$');

        let mut halflen = vec![0; t.len() - 2];
        halflen[1] = 1;

        let mut box_m = 0;
        let mut box_r = 0;
        let (mut c, mut r) = (0, 0);
        for i in 2..halflen.len() {
            let mut hl = 1;
            if i < box_r {
                hl = halflen[box_m * 2 - i].min(box_r - i);
            }
            while t[i - hl] == t[i + hl] {
                hl += 1;
                box_m = i;
                box_r = i + hl;
            }
            halflen[i] = hl;
            if hl > r {
                c = i;
                r = hl;
            }
        }
        s[(c - r + 1) / 2..(c + r - 1) / 2].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    type Func = fn(s: String) -> String;

    fn run_test_case(func: Func, s: String, excepted: String) {
        assert_eq!(excepted, func(s));
    }

    #[test]
    pub fn test_longest_palindrome() {
        let methods: Vec<Func> = vec![
            Solution::longest_palindrome, 
            Solution::longest_palindrome2
        ];

        for method in methods {
            run_test_case(method, String::from("babad"), String::from("bab"));
            run_test_case(method, String::from("cbbd"), String::from("bb"));
        }
    }
}
