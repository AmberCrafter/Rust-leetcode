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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // ref. https://leetcode.com/problems/recover-binary-search-tree/discuss/1963391/Rust-iterative
        // ref. https://www.cnblogs.com/grandyang/p/4297300.html
        // ref. https://www.cnblogs.com/AnnieKim/archive/2013/06/15/morristraversal.html
        // we known that we can get the sorted vector by inorder traversal,
        // asn this problem has two condition:
        // 1. only swap 2 node
        // 2. can't modified the tree structure

        // thus we can use the Morris traversal to redirected the null right left and we can direct to use it
        // here we use a stack to approach this concept.
        // inorder traversal
        let mut stack = vec![];
        let mut curr = root.clone();
        let mut x = None;
        let mut y = None;
        let mut pred: Option<Rc<RefCell<TreeNode>>> = None;

        while !(stack.is_empty() && curr.is_none()) {
            // go to the far left node and record all pass node
            while let Some(node) = curr {
                stack.push(Rc::clone(&node));

                // these three kind of code can reach the same approach but the third one could panic!
                // curr = node.borrow().left.clone();
                curr = if let Some(leaf) = &node.borrow().left {
                    Some(Rc::clone(&leaf))
                } else {
                    None
                }
                // curr = Some(Rc::clone(&node.borrow().left.as_ref().unwrap()));
            }

            // do the inorder traversal and check it is sorted or not
            if let Some(node) = stack.pop() {
                if let Some(p) = pred {
                    if p.borrow().val >= node.borrow().val {
                        y = Some(Rc::clone(&node));
                        if x.is_none() {
                            // get the left hand side error node
                            x = Some(Rc::clone(&p));
                        } else {
                            // get the right hand side error node
                            break;
                        }
                    }
                }
                // update curr and pred
                pred = Some(Rc::clone(&node));

                // these three kind of code can reach the same approach but the third one could panic!
                // curr = node.borrow().right.clone();
                curr = if let Some(leaf) = &node.borrow().right {
                    Some(Rc::clone(&leaf))
                } else {
                    None
                }
                // curr = Some(Rc::clone(&node.borrow().right.as_ref().unwrap()));
            }
        }

        // swap the value
        let mut x = x.as_ref().unwrap().borrow_mut();
        let mut y = y.as_ref().unwrap().borrow_mut();

        let tmp = x.val;
        x.val = y.val;
        y.val = tmp;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut inputs = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        let output = Solution::recover_tree(&mut inputs);
        assert_eq!(except, inputs);
    }

    #[test]
    fn case2() {
        let mut inputs = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        })));
        let except = Some(Rc::new(RefCell::new(TreeNode {
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
        })));
        let output = Solution::recover_tree(&mut inputs);
        assert_eq!(except, inputs);
    }
}
