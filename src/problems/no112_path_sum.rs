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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        fn path_sum(target: i32, tree: Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> bool {
            if let Some(node) = tree.as_ref() {
                let current = node.borrow().val;
                sum += current;
                if target == sum && node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return true;
                }
                path_sum(target, node.borrow().left.clone(), sum)
                    || path_sum(target, node.borrow().right.clone(), sum)
            } else {
                false
            }
        }
        path_sum(target_sum, root, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            TreeNode::layer_gen(vec![5, 4, 8, 11, -99, 13, 4, 7, 2, -99, -99, -99, 1]),
            22,
        );
        let except = true;
        let output = Solution::has_path_sum(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (TreeNode::layer_gen(vec![1, 2, 3]), 5);
        let except = false;
        let output = Solution::has_path_sum(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (TreeNode::layer_gen(vec![]), 0);
        let except = false;
        let output = Solution::has_path_sum(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = (TreeNode::layer_gen(vec![1, 2]), 1);
        let except = false;
        let output = Solution::has_path_sum(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}

use std::collections::VecDeque;
impl TreeNode {
    fn layer_gen(array: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        const NULL_VALUE: i32 = -99;

        let mut root = None;
        let mut idx = 0;
        let mut parent_queue = VecDeque::new();
        while idx < array.len() {
            if root.is_none() {
                root.replace(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                parent_queue.push_back(root.clone());
            } else {
                let node = parent_queue.pop_front().expect("parent_queue is empty");
                let parent = Rc::clone(node.as_ref().unwrap());
                // left leaf
                if array[idx] != NULL_VALUE {
                    parent.borrow_mut().left =
                        Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                    parent_queue.push_back(parent.borrow().left.clone());
                }
                idx += 1;

                if idx < array.len() {
                    // right leaf
                    if array[idx] != NULL_VALUE {
                        parent.borrow_mut().right =
                            Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                        parent_queue.push_back(parent.borrow().right.clone());
                    }
                }
            }
            idx += 1;
        }
        root
    }
}

// best solution
// impl Solution {
//     pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
//         let mut stack = vec![(root, 0)];
//         while let Some((node, sum)) = stack.pop() {
//             if let Some(rc_node) = node {
//                 let node = rc_node.borrow();
//                 let new_sum = sum + node.val;
//                 match new_sum == target_sum && node.left.is_none() && node.right.is_none() {
//                     true => return true,
//                     false => {
//                         stack.push((node.left.clone(), new_sum));
//                         stack.push((node.right.clone(), new_sum));
//                     }
//                 }
//             }
//         }
//         false
//     }
// }
