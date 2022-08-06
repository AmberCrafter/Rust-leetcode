use std::rc::Rc;
use std::cell::RefCell;
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
      right: None
    }
  }
}
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // dfs to find tree height and compare
        fn max_depth(tree: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if tree.is_none() {0}
            else {
                let left = max_depth(tree.as_ref().unwrap().borrow().left.clone());
                let right = max_depth(tree.as_ref().unwrap().borrow().right.clone());
                if (left - right).abs() > 1 || left<0 || right<0 {return -1}
                left.max(right) + 1
            }
        }
        max_depth(root) != -1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = TreeNode::layer_gen(vec![3,9,20,-99,-99,15,7]);
        let except = true;
        let output = Solution::is_balanced(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = TreeNode::layer_gen(vec![1,2,2,3,3,-99,-99,4,4]);
        let except = false;
        let output = Solution::is_balanced(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = TreeNode::layer_gen(vec![1,2,2,3,-99,-99,3,4,-99,-99,4]);
        let except = false;
        let output = Solution::is_balanced(inputs);
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
                if array[idx]!=NULL_VALUE {
                    parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                    parent_queue.push_back(
                        parent.borrow().left.clone()
                    );
                }
                idx+=1;
    
                if idx<array.len() {
                    // right leaf
                    if array[idx]!=NULL_VALUE {
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