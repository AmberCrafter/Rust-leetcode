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
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        // dfs
        fn helper(
            res: &mut Vec<Vec<i32>>,
            tree: &Option<Rc<RefCell<TreeNode>>>,
            path: &mut Vec<i32>,
            sum: &mut i32,
            target_sum: i32,
        ) {
            let mut is_leaf = true;
            let mut val = 0;
            if let Some(leaf) = tree.as_ref().unwrap().borrow().left.clone() {
                is_leaf = false;
                val = (&leaf).borrow().val;
                *sum += val;
                path.push(val);
                helper(res, &Some(leaf), path, sum, target_sum);
                if let Some(ele) = path.pop() {
                    *sum -= ele;
                } else {
                    panic!("Error: vector pop");
                }
            }
            if let Some(leaf) = tree.as_ref().unwrap().borrow().right.clone() {
                is_leaf = false;
                val = (&leaf).borrow().val;
                *sum += val;
                path.push(val);
                helper(res, &Some(leaf), path, sum, target_sum);
                if let Some(ele) = path.pop() {
                    *sum -= ele;
                } else {
                    panic!("Error: vector pop");
                }
            }
            if is_leaf && *sum == target_sum {
                (*res).push(path.clone());
            }
        }

        // init
        let mut res = Vec::new();
        if root.is_some() {
            let mut path = Vec::new();
            let mut sum = root.as_ref().unwrap().borrow().val;
            path.push(root.as_ref().unwrap().borrow().val);
            helper(&mut res, &root, &mut path, &mut sum, target_sum);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            TreeNode::layer_gen(vec![5, 4, 8, 11, -99, 13, 4, 7, 2, -99, -99, 5, 1]),
            22,
        );
        let except = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
        let output = Solution::path_sum(inputs.0, inputs.1);
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
