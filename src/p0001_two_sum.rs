/*

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
*/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut tmp_hash: HashMap<i32, i32> = HashMap::new();
        tmp_hash.insert(nums[0], 0);

        for i in 1..nums.len() {
            let value = target - nums[i];
            if tmp_hash.contains_key(&value) {
                return vec![*tmp_hash.get(&value).unwrap(), i as i32];
            } else {
                tmp_hash.insert(nums[i], i as i32);
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::p0001_two_sum::Solution;

    type TwoSumFn = fn(Vec<i32>, i32) -> Vec<i32>;

    fn run_test_case(two_sum: TwoSumFn, nums: Vec<i32>, target: i32, expected: Vec<i32>) {
        assert_eq!(expected, two_sum(nums, target));
    }

    #[test]
    fn test_two_sum_methods() {
        let methods: Vec<TwoSumFn> = vec![Solution::two_sum, Solution::two_sum_hash];

        for method in methods {
            run_test_case(method, vec![2, 7, 11, 15], 9, vec![0, 1]);
            run_test_case(method, vec![2, 5, 5, 11], 10, vec![1, 2]);
            run_test_case(method, vec![3, 2, 4], 6, vec![1, 2]);
            run_test_case(method, vec![3, 3], 6, vec![0, 1]);
        }
    }
}
