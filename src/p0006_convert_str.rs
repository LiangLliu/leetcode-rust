/*
将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。

比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：

P   A   H   N
A P L S I I G
Y   I   R
之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。

请你实现这个将字符串进行指定行数变换的函数：

string convert(string s, int numRows);

示例 1：

输入：s = "PAYPALISHIRING", numRows = 3
输出："PAHNAPLSIIGYIR"
示例 2：
输入：s = "PAYPALISHIRING", numRows = 4
输出："PINALSIGYAHRPI"
解释：
P     I    N
A   L S  I G
Y A   H R
P     I
示例 3：

输入：s = "A", numRows = 1
输出："A"

 */

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }

        let num_rows_usize = num_rows as usize;
        let mut grid: Vec<String> = vec![String::new(); num_rows_usize];
        let mut current_row = 0;
        let mut going_down = false;

        for ch in s.chars() {
            grid[current_row].push(ch);

            if current_row == 0 || current_row == num_rows_usize - 1 {
                going_down = !going_down;
            }
            if going_down {
                current_row += 1;
            } else {
                current_row -= 1;
            }
        }

        grid.concat()
    }

    pub fn convert1(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }

        let num_rows = num_rows as usize;

        let mut rows = vec![String::new(); num_rows];

        let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();

        iter.zip(s.chars()).for_each(|(i, c)| rows[i].push(c));

        rows.into_iter().collect()
    }
    
    pub fn convert2(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }
        
        let num_rows = num_rows as usize;
        let mut rows: Vec<String> = vec![String::new(); num_rows];
        let mut index = 0;
        let mut step = 1;

        for c in s.chars() {
            rows[index].push(c);
            if index == 0 {
                step = 1;
            } else if index == num_rows - 1 {
                step = -1;
            }
            index = (index as isize + step) as usize;
        }
        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    type Func = fn(s: String, num_rows: i32) -> String;

    fn run_test_case(func: Func, s: String, num_rows: i32, excepted: String) {
        assert_eq!(excepted, func(s, num_rows));
    }

    #[test]
    pub fn test_convert() {
        let methods: Vec<Func> = vec![Solution::convert, Solution::convert1, Solution::convert2];

        for method in methods {
            run_test_case(
                method,
                String::from("PAYPALISHIRING"),
                3,
                String::from("PAHNAPLSIIGYIR"),
            );
            run_test_case(
                method,
                String::from("PAYPALISHIRING"),
                4,
                String::from("PINALSIGYAHRPI"),
            );
            run_test_case(method, String::from("A"), 1, String::from("A"));
            run_test_case(method, String::from("ABC"), 1, String::from("ABC"));
        }
    }
}
