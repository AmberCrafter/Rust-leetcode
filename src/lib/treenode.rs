use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
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

    pub fn layer_gen(array: Vec<i32>, null_value: i32) -> Option<Rc<RefCell<TreeNode>>> {
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

pub trait Generator<T> {
    fn layer_gen(array: Vec<i32>, null_value: i32) -> Option<Rc<RefCell<T>>>;
}

impl Generator<TreeNode> for TreeNode {
    fn layer_gen(array: Vec<i32>, null_value: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // const NULL_VALUE: i32 = -99;

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
                if array[idx] != null_value {
                    parent.borrow_mut().left =
                        Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                    parent_queue.push_back(parent.borrow().left.clone());
                }
                idx += 1;

                if idx < array.len() {
                    // right leaf
                    if array[idx] != null_value {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        let inputs = vec![];
        let expect = None;
        let result = TreeNode::layer_gen(inputs, -99);

        assert_eq!(expect, result);
    }

    #[test]
    fn root_tree() {
        let inputs = vec![0];
        let expect = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let result = TreeNode::layer_gen(inputs, -99);

        assert_eq!(expect, result);
    }

    #[test]
    fn one_layer_full_tree() {
        let inputs = vec![0, 1, 2];
        let expect = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        let result = TreeNode::layer_gen(inputs, -99);

        assert_eq!(expect, result);
    }

    #[test]
    fn one_layer_lefthole_tree() {
        let inputs = vec![0, -99, 2];
        let expect = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        let result = TreeNode::layer_gen(inputs, -99);

        assert_eq!(expect, result);
    }

    #[test]
    fn one_layer_righthole_tree() {
        let inputs = vec![0, 1];
        let expect = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: None,
        })));
        let result = TreeNode::layer_gen(inputs, -99);

        assert_eq!(expect, result);
    }
}
