use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
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

use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // bfs
        // maintain a next node queue
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        let mut is_reverse = false;
        queue.push_back(vec![root.clone()]);

        while let Some(layer) = queue.pop_front() {
            let mut next_layer = Vec::new();
            let mut curr_layer = Vec::new();
            for node in layer {
                if let Some(node) = node.as_ref() {
                    curr_layer.push(node.borrow().val);
                    if is_reverse {
                        if let Some(next) = node.borrow().right.clone() {
                            next_layer.push(Some(next));
                        }
                        if let Some(next) = node.borrow().left.clone() {
                            next_layer.push(Some(next));
                        }
                    } else {
                        if let Some(next) = node.borrow().left.clone() {
                            next_layer.push(Some(next));
                        }
                        if let Some(next) = node.borrow().right.clone() {
                            next_layer.push(Some(next));
                        }
                    }
                }
            }

            is_reverse = !is_reverse;
            next_layer.reverse();

            if next_layer.len() > 0 {
                queue.push_back(next_layer);
            }
            if curr_layer.len() > 0 {
                result.push(curr_layer);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let except = vec![vec![3], vec![20, 9], vec![15, 7]];
        let output = Solution::zigzag_level_order(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let except = vec![vec![1], vec![3, 2], vec![4, 5]];
        let output = Solution::zigzag_level_order(inputs);
        assert_eq!(except, output);
    }
}
