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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // bfs
        if root.is_none() {return 0}
        let mut layer_buf = None;
        let mut count = 0;
        layer_buf.replace(vec![root.clone()]);
        while let Some(layer) = layer_buf.take() {
            let mut next_layer = Vec::new();
            for node in layer {
                if let Some(node) = node.as_ref() {
                    if let Some(next) = node.borrow().left.clone() {
                        next_layer.push(Some(next));
                    }
                    if let Some(next) = node.borrow().right.clone() {
                        next_layer.push(Some(next));
                    }
                }
            }
            if !next_layer.is_empty() {
                layer_buf.replace(next_layer);
            }
            count+=1;
        }
        count
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
                left: None, right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None, right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None, right: None
                })))
            })))
        })));
        let except = 3;
        let output = Solution::max_depth(inputs);
        assert_eq!(except, output);
    }
}