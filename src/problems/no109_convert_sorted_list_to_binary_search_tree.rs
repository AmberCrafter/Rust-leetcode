use std::cell::RefCell;
use std::rc::Rc;
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
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut head = head;
        let mut nums = Vec::new();
        while let Some(node) = head {
            nums.push(node.val);
            head = node.next;
        }
        fn builder(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match nums.len() {
                0 => None,
                1 => Some(Rc::new(RefCell::new(TreeNode::new(nums[0])))),
                _ => {
                    let pivot = nums.len() / 2;
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: nums[pivot],
                        left: builder(&nums[..pivot]),
                        right: builder(&nums[pivot + 1..]),
                    })))
                }
            }
        }
        builder(&nums)
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn case1() {
//         let inputs = None;
//         let except = None;
//         let output = None;
//         assert_eq!(except, output);
//     }
// }
