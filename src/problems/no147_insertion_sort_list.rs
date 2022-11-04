use std::ops::Deref;

pub struct Solution {}
// Definition for singly-linked list.
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
impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // ref best solution: https://leetcode.com/problems/insertion-sort-list/discuss/2739687/Rust

        let mut dummy = ListNode::new(0);
        while let Some(mut node) = head {
            head = node.as_mut().next.take();
            let mut prev = &mut dummy;
            while prev.next.is_some() && prev.next.as_ref().unwrap().val < node.val {
                prev = prev.next.as_mut().unwrap();
            }
            node.next = prev.next.take();
            prev.next = Some(node);
        }
        dummy.next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = ListNode::gen(vec![4, 2, 1, 3]);
        let except = ListNode::gen(vec![1, 2, 3, 4]);
        let output = Solution::insertion_sort_list(inputs);
        assert_eq!(except, output);
    }
}

impl ListNode {
    fn gen(arr: Vec<i32>) -> Option<Box<Self>> {
        let mut root = Some(Box::new(ListNode::new(0)));
        let mut cur = root.as_mut().unwrap();
        for val in arr {
            cur.next.replace(Box::new(ListNode::new(val)));
            cur = cur.next.as_mut().unwrap();
        }
        root.unwrap().next
    }
}
