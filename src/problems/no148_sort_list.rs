pub struct Solution {}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
pub val: i32,
pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
        next: None,
        val
        }
    }
}
impl Solution {
    // ref: https://leetcode.com/problems/sort-list/discuss/2525275/Rust-Merge-Sort
    fn merge(mut left: Option<Box<ListNode>>, mut right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (left, right) {
            (None, None) => None,
            (Some(lnode), None) => Some(lnode),
            (None, Some(rnode)) => Some(rnode),
            (Some(lnode), Some(rnode)) => {
                if lnode.val<rnode.val {
                    Some(Box::new(ListNode {val: lnode.val, next: Self::merge(lnode.next, Some(rnode))}))
                } else {
                    Some(Box::new(ListNode { val: rnode.val, next: Self::merge(Some(lnode), rnode.next) }))
                }
            }
        }
    }
    fn merge_sort(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {return head;}
        // find mid point
        let tmp_head = head.clone();
        let mut fast = tmp_head.as_ref().unwrap().next.as_ref();
        let mut slow = head.as_mut();
        while fast.is_some() && fast.unwrap().next.is_some() {
            slow = slow.unwrap().next.as_mut();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        let mid = slow.unwrap().next.take();

        let left = Self::merge_sort(head);
        let right = Self::merge_sort(mid);
        Self::merge(left,right)
    }
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::merge_sort(head)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = ListNode::gen(vec![4, 2, 1, 3]);
        let except = ListNode::gen(vec![1, 2, 3, 4]);
        let output = Solution::sort_list(inputs);
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