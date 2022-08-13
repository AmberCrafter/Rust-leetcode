use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;
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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // use dfs to figure out each node depth
        fn dfs(tree: Option<Rc<RefCell<TreeNode>>>, layer: i32, mindepth: &mut i32) -> i32 {
            if tree.is_none() {
                -1
            } else {
                // left node
                let left = dfs(
                    tree.as_ref().unwrap().borrow().left.clone(),
                    layer + 1,
                    mindepth,
                );
                // right node
                let right = dfs(
                    tree.as_ref().unwrap().borrow().right.clone(),
                    layer + 1,
                    mindepth,
                );

                if left == -1 && right == -1 {
                    *mindepth = (*mindepth).min(layer);
                }
                layer
            }
        }

        let mut mindepth = i32::MAX;
        match dfs(root, 1, &mut mindepth) {
            -1 => 0,
            _ => mindepth,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = TreeNode::layer_gen(vec![3, 9, 20, -99, -99, 15, 7]);
        let except = 2;
        let output = Solution::min_depth(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case2() {
        let inputs = TreeNode::layer_gen(vec![]);
        let except = 0;
        let output = Solution::min_depth(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case3() {
        let inputs = TreeNode::layer_gen(vec![2, -99, 3, -99, 4, -99, 5, -99, 6]);
        let except = 5;
        let output = Solution::min_depth(inputs);
        assert_eq!(except, output);
    }
}

use std::collections::VecDeque;
impl TreeNode {
    fn layer_gen(array: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        const NULL_VALUE: i32 = -99;

        let mut root = None;
        let mut idx = 0;
        let mut parent_queue = VecDeque::new();
        while idx < array.len() {
            if root.is_none() {
                root.replace(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                parent_queue.push_back(root.clone());
            } else {
                let node = parent_queue.pop_front().expect("parent_queue is empty");
                let parent = Rc::clone(node.as_ref().unwrap());
                // left leaf
                if array[idx] != NULL_VALUE {
                    parent.borrow_mut().left =
                        Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                    parent_queue.push_back(parent.borrow().left.clone());
                }
                idx += 1;

                if idx < array.len() {
                    // right leaf
                    if array[idx] != NULL_VALUE {
                        parent.borrow_mut().right =
                            Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                        parent_queue.push_back(parent.borrow().right.clone());
                    }
                }
            }
            idx += 1;
        }
        root
    }
}
