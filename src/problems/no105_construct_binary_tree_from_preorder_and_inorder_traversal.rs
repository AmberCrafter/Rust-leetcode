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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn gen_right_tree(
            parent_stack: &mut Vec<i32>,
            preorder: &Vec<i32>,
            inorder: &Vec<i32>,
            pdx: &mut usize,
            idx: &mut usize,
            mut be_parent: bool
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let mut curr = None;
            while *pdx<preorder.len() {
                // find parent
                if let Some(parent) = parent_stack.pop() {
                    if parent == inorder[*idx] && be_parent {
                        *idx+=1;
                        return Some(Rc::new(RefCell::new(TreeNode {
                            val: inorder[*idx-1],
                            left: curr.clone(),
                            right: gen_right_tree(
                                parent_stack,
                                preorder,
                                inorder,
                                pdx,
                                idx,
                                be_parent
                            ),
                        })))
                    } else {
                        parent_stack.push(parent);
                    }
                }
                // left leaf
                if *pdx>=preorder.len() {break}
                be_parent = if parent_stack.is_empty() {
                    parent_stack.push(preorder[*pdx]);
                    true
                } else {
                    if preorder[*pdx] != inorder[*idx] {
                        parent_stack.push(preorder[*pdx]);
                        true
                    } else {
                        curr = Some(Rc::new(RefCell::new(TreeNode::new(preorder[*pdx]))));
                        *idx += 1;
                        // next element is right node case
                        if *pdx<preorder.len()-1 {
                            if preorder[*pdx+1]==inorder[*idx] && be_parent {
                                let tree = curr.clone();
                                tree.unwrap().borrow_mut().right.replace(Rc::new(RefCell::new(TreeNode::new(preorder[*pdx+1]))));
                                *pdx+=1;
                                *idx+=1;
                            }
                        }
                        true
                    }
                };
                
                *pdx += 1;
            }
            // consume residual stack as parents
            while let Some(val) = parent_stack.pop() {
                curr = Some(Rc::new(RefCell::new(TreeNode { val, left: curr, right: None })))
            }
            curr
        }

        let mut parent_stack = Vec::new();
        let mut pdx = 0;
        let mut idx = 0;
        
        gen_right_tree(&mut parent_stack, &preorder, &inorder, &mut pdx, &mut idx, false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (vec![3,9,20,15,7], vec![9,3,15,20,7]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
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
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (vec![1,2], vec![2,1]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None, right: None
            }))),
            right: None
        })));
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (vec![1,2], vec![1,2]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None, right: None
            }))),
        })));
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = (vec![1,2,3], vec![3,2,1]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None, right: None
                }))),
                right: None
            }))),
            right: None,
        })));
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case5() {
        let inputs = (vec![1,2,3], vec![2,3,1]);
        let except = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None, right: None
                })))
            }))),
            right: None,
        })));
        let output = Solution::build_tree(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
