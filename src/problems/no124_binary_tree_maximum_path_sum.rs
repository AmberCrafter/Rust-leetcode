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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MIN;
        let mut left = 0;
        let mut right = 0;
        let cur = root.as_ref().unwrap().borrow().val;

        // postorder_traversal
        if let Some(node) = root.as_ref().unwrap().borrow().left.clone() {
            res = res.max(Solution::max_path_sum(Some(node.clone())));
            left = node.borrow().val;
        }
        if let Some(node) = root.as_ref().unwrap().borrow().right.clone() {
            res = res.max(Solution::max_path_sum(Some(node.clone())));
            right = node.borrow().val;
        }

        // println!();
        // println!();
        // println!("==================================================");
        // println!("{:#?}", root);
        // println!("==================================================");

        res = res
            .max(cur)
            .max(cur+left)
            .max(cur+right)
            .max(
                cur+left+right
            );
        
        root.as_ref().unwrap().borrow_mut().val = cur + 0.max(left).max(right);

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = TreeNode::layer_gen(vec![1, 2, 3], -99);
        let except = 6;
        let output = Solution::max_path_sum(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = TreeNode::layer_gen(vec![-10,9,20,-99,-99,15,7], -99);
        let except = 42;
        let output = Solution::max_path_sum(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = TreeNode::layer_gen(vec![1,2,-99,3,-99,4,-99,5], -99);
        // println!("{:#?}",inputs);
        let except = 15;
        let output = Solution::max_path_sum(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = TreeNode::layer_gen(vec!
            [9,6,-3,-99,-99,-6,2,-99,-99,2,-99,-6,-6,-6], -99);
        // println!("{:#?}",inputs);
        let except = 16;
        let output = Solution::max_path_sum(inputs);
        assert_eq!(except, output);
    }
}

use std::collections::VecDeque;
impl TreeNode {
    fn layer_gen(array: Vec<i32>, null_value: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // const NULL_VALUE: i32 = -99;

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
                if array[idx] != null_value {
                    parent.borrow_mut().left =
                        Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                    parent_queue.push_back(parent.borrow().left.clone());
                }
                idx += 1;

                if idx < array.len() {
                    // right leaf
                    if array[idx] != null_value {
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
