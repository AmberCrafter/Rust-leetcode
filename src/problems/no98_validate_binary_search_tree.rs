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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // dfs traveler
        fn checker(tree: Option<Rc<RefCell<TreeNode>>>, smaller: &mut Vec<i32>, larger: &mut Vec<i32>) -> bool {
            if let Some(node) = tree {
                println!("{:?}", node);
                let node = node.borrow();

                if smaller.iter().any(|v| v<=&node.val) ||
                    larger.iter().any(|v| v>=&node.val) {
                        return false
                    }

                // if node.val
                //     < node
                //         .left
                //         .as_ref()
                //         .unwrap_or(&Rc::new(RefCell::new(TreeNode::new(i32::MIN))))
                //         .borrow()
                //         .val
                //     || node.val
                //         > node
                //             .right
                //             .as_ref()
                //             .unwrap_or(&Rc::new(RefCell::new(TreeNode::new(i32::MAX))))
                //             .borrow()
                //             .val
                // {
                //     return false;
                // }
                (if let Some(rcnode) = node.left.as_ref() {
                    smaller.push(node.val);
                    let res = checker(Some(Rc::clone(rcnode)), smaller, larger);
                    smaller.pop();
                    res
                } else {
                    true
                }) && (if let Some(rcnode) = node.right.as_ref() {
                    larger.push(node.val);
                    let res = checker(Some(Rc::clone(rcnode)), smaller, larger);
                    larger.pop();
                    res
                } else {
                    true
                })
            } else {
                true
            }
        }
        checker(root, &mut vec![], &mut vec![])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = Some(Rc::new(RefCell::new(TreeNode {
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
        let except = true;
        let output = Solution::is_valid_bst(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let except = false;
        let output = Solution::is_valid_bst(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = Some(Rc::new(RefCell::new(TreeNode {
            val: 2147483647,
            left: None,
            right: None,
        })));
        let except = true;
        let output = Solution::is_valid_bst(inputs);
        assert_eq!(except, output);
    }
}
