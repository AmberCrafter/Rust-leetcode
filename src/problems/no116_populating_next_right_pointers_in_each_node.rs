use std::rc::Rc;
use std::cell::RefCell;
pub struct Solution {}
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
  pub next: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
      next: None,
    }
  }
}

use std::collections::VecDeque;
impl Solution {
    pub fn connect(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // layer traversal
        // bfs
        let mut layer = VecDeque::new();
        layer.push_back(root.clone());
        while !layer.is_empty() {
            let mut next_layer = VecDeque::new();
            let mut next = None;
            for pnode in layer.iter().rev() {
                if let Some(leaf) = pnode.as_ref().unwrap().borrow().left.clone() {
                    next_layer.push_back(Some(leaf));
                }
                if let Some(leaf) = pnode.as_ref().unwrap().borrow().right.clone() {
                    next_layer.push_back(Some(leaf));
                }
                pnode.as_ref().unwrap().borrow_mut().next = next;
                next = Some(Rc::clone(pnode.as_ref().unwrap()));
            }
            layer = next_layer;
        }
        println!("{:?}",root);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut inputs = TreeNode::layer_gen(vec![1,2,3], -99);
        let except = None;
        let output = Solution::connect(&mut inputs);
        assert_eq!(except, inputs);
    }
}

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