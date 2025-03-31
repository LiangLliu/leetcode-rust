/*
    给你一个下标从 0 开始、长度为 n 的二进制字符串 s ，你可以对其执行两种操作：

    选中一个下标 i 并且反转从下标 0 到下标 i（包括下标 0 和下标 i ）的所有字符，成本为 i + 1 。
    选中一个下标 i 并且反转从下标 i 到下标 n - 1（包括下标 i 和下标 n - 1 ）的所有字符，成本为 n - i 。
    返回使字符串内所有字符 相等 需要的 最小成本 。

    反转 字符意味着：如果原来的值是 '0' ，则反转后值变为 '1' ，反之亦然。
*/

struct Solution;

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let len = s.len();
        s.chars()
            .collect::<Vec<_>>()
            .windows(2)
            .enumerate()
            .filter(|&(_, w)| w[0] != w[1])
            .map(|(i, _)| (i + 1).min(len - (i + 1)) as i64)
            .sum()
    }

    pub fn minimum_cost2(s: String) -> i64 {
        let n = s.len();
        if n <= 1 {
            return 0;
        }
        let mut ans = 0;

        let s = s.as_bytes();
        for i in 1..n {
            if s[i] != s[i - 1] {
                ans += i.min(n - i);
            }
        }
        ans as i64
    }
    pub fn minimum_cost3(s: String) -> i64 {
        let s: Vec<_> = s.chars().collect();
        let n = s.len();
        let mut suf = vec![vec![0; 2]; n + 1];
        for i in (0..n).rev() {
            if s[i] == '1' {
                suf[i][1] = suf[i + 1][1];
                suf[i][0] = suf[i + 1][1] + (n - i) as i64;
            } else {
                suf[i][1] = suf[i + 1][0] + (n - i) as i64;
                suf[i][0] = suf[i + 1][0];
            }
        }
        let mut pre = vec![0; 2];
        let mut res = i64::MAX;
        for i in 0..n {
            if s[i] == '1' {
                pre[0] = pre[1] + (i + 1) as i64;
            } else {
                pre[1] = pre[0] + (i + 1) as i64;
            }
            res = res.min(std::cmp::min(
                pre[0] + suf[i + 1][0],
                pre[1] + suf[i + 1][1],
            ))
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::p2712_minimum_cost::Solution;

    type Func = fn(s: String) -> i64;

    fn run_test_case(func: Func, s: String, expected: i64) {
        assert_eq!(expected, func(s));
    }

    #[test]
    fn test_minimum_cost() {
        let methods: Vec<Func> = vec![Solution::minimum_cost, Solution::minimum_cost2];

        for method in methods {
            run_test_case(method, String::from(""), 0);
            run_test_case(method, String::from("0011"), 2);
            run_test_case(method, String::from("010101"), 9);
        }
    }
}
