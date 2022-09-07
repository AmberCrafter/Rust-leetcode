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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // dfs
        // need to ensure all node values are between 0-9
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, buffer: &mut Vec<i32>, value: &mut i32) {
            *value = *value * 10 + root.as_ref().unwrap().borrow().val;
            // This node is leaf
            if root.as_ref().unwrap().borrow().left.is_none() && root.as_ref().unwrap().borrow().right.is_none() {
                buffer.push(*value);
                *value/=10;
                return;
            }

            if let Some(node) = root.as_ref().unwrap().borrow().left.clone() {
                dfs(Some(node), buffer, value);
            }
            
            if let Some(node) = root.as_ref().unwrap().borrow().right.clone() {
                dfs(Some(node), buffer, value);
            }

            *value/=10;
        }

        let mut buffer = Vec::new();
        let mut value = 0;

        dfs(root, &mut buffer, &mut value);

        // println!("{:?}", buffer);
        buffer.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = TreeNode::layer_gen(vec![1,2,3], -99);
        let except = 25;
        let output = Solution::sum_numbers(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = TreeNode::layer_gen(vec![4,9,0,5,1], -99);
        let except = 1026;
        let output = Solution::sum_numbers(inputs);
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
        while idx<array.len() {
            if root.is_none() {
                root.replace(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                parent_queue.push_back(
                    root.clone()
                );
            } else {
                let node = parent_queue.pop_front().expect("parent_queue is empty");
                let parent = Rc::clone(node.as_ref().unwrap());
                // left leaf
                if array[idx]!=null_value {
                    parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                    parent_queue.push_back(
                        parent.borrow().left.clone()
                    );
                }
                idx+=1;
    
                if idx<array.len() {
                    // right leaf
                    if array[idx]!=null_value {
                        parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                        parent_queue.push_back(
                            parent.borrow().right.clone()
                        );
                    }
                }
            }
            idx+=1;
        }
        root 
    }
}