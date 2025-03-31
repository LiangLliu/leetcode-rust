/*
   给你一个 n 个节点的 有向图 ，节点编号为 0 到 n - 1 ，其中每个节点 至多 有一条出边。
   图用一个大小为 n 下标从 0 开始的数组 edges 表示，节点 i 到节点 edges[i] 之间有一条有向边。如果节点 i 没有出边，那么 edges[i] == -1 。
   请你返回图中的 最长 环，如果没有任何环，请返回 -1 。
   一个环指的是起点和终点是 同一个 节点的路径。
*/

struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut max_length = -1;
        let mut time = 1;

        let mut visit_time: Vec<i32> = vec![0; n];

        for i in 0..n {
            if visit_time[i] > 0 {
                continue;
            }

            let start_time = time;
            let mut curr_node_idx = i as i32;

            while curr_node_idx != -1 {
                let current_node_usize = curr_node_idx as usize;

                if visit_time[current_node_usize] > 0 {
                    if visit_time[current_node_usize] >= start_time {
                        let cycle_len = time - visit_time[current_node_usize];

                        max_length = max_length.max(cycle_len);
                    }
                    break;
                }

                visit_time[current_node_usize] = time;
                time += 1;
                curr_node_idx = edges[current_node_usize];
            }
        }
        max_length
    }
}

#[cfg(test)]
mod tests {
    use crate::p2360_longest_cycle::Solution;

    type Func = fn(edges: Vec<i32>) -> i32;

    fn run_test_case(func: Func, edges: Vec<i32>, expected: i32) {
        let actual = func(edges);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_longest_cycle() {
        let methods: Vec<Func> = vec![Solution::longest_cycle];

        for method in methods {
            run_test_case(method, Vec::new(), -1);
            run_test_case(method, vec![3, 3, 4, 2, 3], 3);
        }
    }
}
