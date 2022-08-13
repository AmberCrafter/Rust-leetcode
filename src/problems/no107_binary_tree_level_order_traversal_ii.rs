use std::cell::RefCell;
use std::collections::VecDeque;
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
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut layer = Vec::new();
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        };

        layer.push(root);
        while !layer.is_empty() {
            let mut next_layer = Vec::new();
            let mut tmp = Vec::new();
            for node in layer {
                if node.is_none() {
                    continue;
                }
                tmp.push(node.as_ref().unwrap().borrow().val);
                if let Some(leaf) = node.as_ref().unwrap().borrow().left.clone() {
                    next_layer.push(Some(leaf));
                };
                if let Some(leaf) = node.as_ref().unwrap().borrow().right.clone() {
                    next_layer.push(Some(leaf));
                };
            }
            layer = next_layer;
            result.push(tmp);
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        use crate::lib::treenode;
        let inputs = TreeNode::layer_gen(vec![3, 9, 20, -99, -99, 15, 7]);
        let except = vec![vec![15, 7], vec![9, 20], vec![3]];
        let output = Solution::level_order_bottom(inputs);
        assert_eq!(except, output);
    }
}

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
