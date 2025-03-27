/*
    给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。

    算法的时间复杂度应该为 O(log (m+n)) 。
*/

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let total_len = len1 + len2;

        if total_len == 0 {
            return 0.0;
        }

        let mut idx1 = 0;
        let mut idx2 = 0;
        let mut current_val = 0;
        let mut prev_val = 0;

        for _ in 0..=(total_len / 2) {
            prev_val = current_val;

            let val1 = nums1.get(idx1).cloned();
            let val2 = nums2.get(idx2).cloned();

            match (val1, val2) {
                (Some(n1), Some(n2)) => {
                    if n1 < n2 {
                        current_val = n1;
                        idx1 += 1;
                    } else {
                        current_val = n2;
                        idx2 += 1;
                    }
                }
                (Some(n1), None) => {
                    current_val = n1;
                    idx1 += 1;
                }
                (None, Some(n2)) => {
                    current_val = n2;
                    idx2 += 1;
                }
                (None, None) => {
                    break;
                }
            }
        }

        if total_len % 2 == 1 {
            current_val as f64
        } else {
            (prev_val as f64 + current_val as f64) / 2.0
        }
    }

    pub fn find_median_sorted_arrays2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let total_len = len1 + len2;

        if total_len == 0 {
            return 0.0;
        }

        let min_left = (total_len + 1) / 2;
        let min_right = (total_len + 2) / 2;

        fn binary_search(v1: Vec<i32>, i: usize, v2: Vec<i32>, j: usize, k: usize) -> i32 {
            if i >= v1.len() {
                return v2[j + k - 1];
            }
            if j >= v2.len() {
                return v1[i + k - 1];
            }
            if k == 1 {
                return v1[i].min(v2[j]);
            }
            let max1 = if (i + k / 2 - 1) < v1.len() {
                v1[i + k / 2 - 1]
            } else {
                i32::MAX
            };
            let max2 = if (j + k / 2 - 1) < v2.len() {
                v2[j + k / 2 - 1]
            } else {
                i32::MAX
            };

            if max1 > max2 {
                binary_search(v1, i, v2, j + k / 2, k - k / 2)
            } else {
                binary_search(v1, i + k / 2, v2, j, k - k / 2)
            }
        }

        let left = binary_search(nums1.clone(), 0, nums2.clone(), 0, min_left);
        let right = binary_search(nums1, 0, nums2.clone(), 0, min_right);

        (left + right) as f64 / 2.0
    }
}

#[cfg(test)]
mod tests {
    use crate::p0004_find_median_sorted_arrays::Solution;

    type Func = fn(nums1: Vec<i32>, nums2: Vec<i32>) -> f64;

    fn run_test_case(func: Func, nums1: Vec<i32>, nums2: Vec<i32>, expected: f64) {
        assert_eq!(expected, func(nums1, nums2));
    }

    #[test]
    fn test_two_sum_methods() {
        let methods: Vec<Func> = vec![
            Solution::find_median_sorted_arrays,
            Solution::find_median_sorted_arrays2,
        ];

        for method in methods {
            run_test_case(method, Vec::from([]), Vec::from([]), 0.0);
            run_test_case(method, Vec::from([]), Vec::from([1]), 1.0);
            run_test_case(method, Vec::from([1, 3]), Vec::from([2]), 2.00000);
            run_test_case(method, Vec::from([1, 2]), Vec::from([3, 4]), 2.50000);
            run_test_case(method, Vec::from([100000]), Vec::from([100001]), 100000.5);
        }
    }
}
