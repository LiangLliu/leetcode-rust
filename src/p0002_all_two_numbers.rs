/*
给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。

请你将两个数相加，并以相同形式返回一个表示和的链表。

你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
*/

/*
    Definition for singly-linked list.
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn create_linked_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for &val in arr {
        let new_node = Some(Box::new(ListNode::new(val)));
        *tail = new_node;
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn linked_list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = list.as_ref();

    while let Some(node) = current {
        result.push(node.val);
        current = node.next.as_ref();
    }

    result
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut l1_current = l1.as_ref();
        let mut l2_current = l2.as_ref();
        let mut head = None;
        let mut current = &mut head;

        while l1_current.is_some() || l2_current.is_some() || carry > 0 {
            let sum = l1_current.map_or(0, |node| node.val)
                + l2_current.map_or(0, |node| node.val)
                + carry;

            carry = sum / 10;
            let digit = sum % 10;

            *current = Some(Box::new(ListNode {
                val: digit,
                next: current.take(),
            }));
            current = &mut current.as_mut().unwrap().next;

            l1_current = l1_current.and_then(|node| node.next.as_ref());
            l2_current = l2_current.and_then(|node| node.next.as_ref());
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use crate::p0002_all_two_numbers::{ListNode, Solution, create_linked_list, linked_list_to_vec};

    type TestFn = fn(Option<Box<ListNode>>, Option<Box<ListNode>>) -> Option<Box<ListNode>>;

    fn run_test_case(add_two_numbers: TestFn, l1: Vec<i32>, l2: Vec<i32>, expected: Vec<i32>) {
        let list1 = create_linked_list(&l1);
        let list2 = create_linked_list(&l2);

        let result = linked_list_to_vec(add_two_numbers(list1, list2));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_two_sum_methods() {
        let methods: Vec<TestFn> = vec![Solution::add_two_numbers];

        for method in methods {
            run_test_case(method, vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]);
            run_test_case(method, vec![0], vec![0], vec![0]);
            run_test_case(
                method,
                vec![9, 9, 9, 9, 9, 9, 9],
                vec![9, 9, 9, 9],
                vec![8, 9, 9, 9, 0, 0, 0, 1],
            );
        }
    }
}
