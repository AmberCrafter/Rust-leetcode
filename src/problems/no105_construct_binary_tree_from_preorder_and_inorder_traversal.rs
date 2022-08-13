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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // ref. https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/discuss/962765/Rust-recursive
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn builder(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(pivot) = inorder.iter().position(|&v| v == preorder[0]) {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: preorder[0],
                    left: builder(&preorder[1..(1 + pivot)], &inorder[..pivot]),
                    right: builder(&preorder[(1 + pivot)..], &inorder[(pivot + 1)..]),
                })))
            } else {
                None
            }
        }
        builder(&preorder[..], &inorder[..])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
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

    #[test]
    fn case2() {
        let inputs = (vec![1, 2], vec![2, 1]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })));
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (vec![1, 2], vec![1, 2]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = (vec![1, 2, 3], vec![3, 2, 1]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: None,
        })));
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case5() {
        let inputs = (vec![1, 2, 3], vec![2, 3, 1]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        })));
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
