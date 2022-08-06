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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn builder(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match nums.len() {
                0 => None,
                1 => Some(Rc::new(RefCell::new(TreeNode::new(nums[0])))),
                _ => {
                    let pivot = nums.len()/2;
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: nums[pivot],
                        left: builder(&nums[..pivot]),
                        right: builder(&nums[pivot+1 ..])
                    })))
                }
            }
        }
        builder(&nums)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![-10,-3,0,5,9];
        let except = TreeNode::layer_gen(vec![0,-3,9,-10,-99,5]);
        let output = Solution::sorted_array_to_bst(inputs);
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