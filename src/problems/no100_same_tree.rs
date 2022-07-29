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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // dfs checker
        // iterative
        let mut ps = Vec::new();
        let mut qs = Vec::new();
        let mut pc = p.clone();
        let mut qc = q.clone();

        while !(pc.is_none() && qc.is_none() && ps.is_empty() && qs.is_empty()) {
            if pc.is_some() ^ qc.is_some() {return false};
            if let (Some(pnode), Some(qnode)) = (pc, qc) {
                if pnode.borrow().val!=qnode.borrow().val {
                    return false
                }
                ps.push(Rc::clone(&pnode));
                qs.push(Rc::clone(&qnode));
                pc = pnode.borrow().left.clone();
                qc = qnode.borrow().left.clone();
            } else {
                pc = ps.pop().unwrap().borrow().right.clone();
                qc = qs.pop().unwrap().borrow().right.clone();
            }
        }
        return true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None, right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None, right: None,
            })))
        }))),Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None, right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None, right: None,
            })))
        }))));
        let except = true;
        let output = Solution::is_same_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}