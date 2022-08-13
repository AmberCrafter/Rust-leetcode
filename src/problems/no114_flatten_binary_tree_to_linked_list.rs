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
    fn recusive(
        root: Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) {
        // record
        path.push(Some(Rc::clone(root.as_ref().unwrap())));
        if let Some(next) = root.as_ref().unwrap().borrow().left.as_ref() {
            Solution::recusive(Some(Rc::clone(next)), path)
        }
        if let Some(next) = root.as_ref().unwrap().borrow().right.as_ref() {
            Solution::recusive(Some(Rc::clone(next)), path)
        }
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // flatten order: pre-order
        if root.is_some() {
            let mut path = Vec::new();
            Solution::recusive(root.clone(), &mut path);
            // rebuild the tree as linked list
            let mut curr = None;
            while let Some(parent) = path.pop() {
                parent.as_ref().unwrap().borrow_mut().left.take();
                parent.as_ref().unwrap().borrow_mut().right = curr;
                curr = parent;
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut inputs = TreeNode::layer_gen(vec![1, 2, 5, 3, 4, -99, 6]);
        let except = TreeNode::layer_gen(vec![1, -99, 2, -99, 3, -99, 4, -99, 5, -99, 6]);
        let output = Solution::flatten(&mut inputs);
        assert_eq!(except, inputs);
    }
    #[test]
    fn case2() {
        let mut inputs = TreeNode::layer_gen(vec![]);
        let except = TreeNode::layer_gen(vec![]);
        let output = Solution::flatten(&mut inputs);
        assert_eq!(except, inputs);
    }
    #[test]
    fn case3() {
        let mut inputs = TreeNode::layer_gen(vec![1]);
        let except = TreeNode::layer_gen(vec![1]);
        let output = Solution::flatten(&mut inputs);
        assert_eq!(except, inputs);
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
