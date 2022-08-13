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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn builder(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(pivot) = inorder
                .iter()
                .position(|v| v == postorder.iter().last().unwrap())
            {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: inorder[pivot],
                    left: builder(&inorder[..pivot], &postorder[..pivot]),
                    right: builder(
                        &inorder[pivot + 1..],
                        &postorder[pivot..postorder.len() - 1],
                    ),
                })))
            } else {
                None
            }
        }
        builder(&inorder, &postorder)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
