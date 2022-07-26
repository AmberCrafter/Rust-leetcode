pub struct Solution {}
use std::rc::Rc;
use std::cell::RefCell;
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn unpack(stack: &mut Vec<i32>, tree: Rc<RefCell<TreeNode>>) {
            if let Some(node) = &tree.borrow().left {
                unpack(stack, Rc::clone(node));
            }
            if let Some(node) = &tree.borrow().right {
                unpack(stack, Rc::clone(node));
            }
            stack.push(tree.borrow().val);
        }
        let mut res = Vec::new();
        if let Some(node) = root {
            unpack(&mut res, node);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        })));
        let except = vec![3,2,1];
        let output = Solution::postorder_traversal(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = None;
        let except = Vec::<i32>::new();
        let output = Solution::postorder_traversal(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: None,
            right: None
        })));
        let except = vec![1];
        let output = Solution::postorder_traversal(inputs);
        assert_eq!(except, output);
    }
}