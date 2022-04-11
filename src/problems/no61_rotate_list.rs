use std::ops::Deref;

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // ref. https://leetcode.com/problems/rotate-list/discuss/883433/Rust-0ms
        // Rust is not easy to maintain ListNode structure with ownership system
        // Thus, creating the new ListNode is the more performance way than modified origin Node
        let mut arr: Vec<i32> = Vec::new();
        let mut node = head.as_ref();
        while let Some(n) = node {
            arr.push(n.val);
            node = n.next.as_ref();
        }

        let mut res = None;
        for i in (0..arr.len()).rev() {
            let j = k as usize % arr.len();
            res = Some(Box::new(ListNode {
                val: arr[(arr.len() + i - j) % arr.len()],
                next: res,
            }))
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let listnode = Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        });
        let inputs = (listnode, 2);
        let expected = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
            })),
        }));
        let result = Solution::rotate_right(Some(inputs.0), inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let listnode = Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: None
                })),
            })),
        });
        let inputs = (listnode, 4);
        let expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: None
                })),
            })),
        }));
        let result = Solution::rotate_right(Some(inputs.0), inputs.1);

        assert_eq!(expected, result);
    }
}
