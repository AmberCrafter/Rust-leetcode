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
    // ref. https://leetcode.com/problems/unique-binary-search-trees-ii/discuss/957115/Rust-Solution
    // due to we can't peek the RefCell value, nor deep clone rc refcell data easily. Thus we need to build the tree botton up. (DFS method)
    fn helper(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if l == r {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(l))))];
        }

        let mut res = vec![];
        for pivot in l..=r {
            let l_trees = if pivot == l {
                vec![None]
            } else {
                Self::helper(l, pivot - 1)
            };
            let r_trees = if pivot == r {
                vec![None]
            } else {
                Self::helper(pivot + 1, r)
            };
            for i in 0..l_trees.len() {
                for j in 0..r_trees.len() {
                    let tree = TreeNode {
                        val: pivot,
                        left: l_trees[i].clone(),
                        right: r_trees[j].clone(),
                    };
                    res.push(Some(Rc::new(RefCell::new(tree))));
                }
            }
        }
        res
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::helper(1, n)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = 3;
        let except = vec![
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: None,
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
        ];
        let output = Solution::generate_trees(inputs);
        assert_eq!(except, output);
    }
}
