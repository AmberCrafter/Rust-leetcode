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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut smaller = Vec::new();
        let mut larger = Vec::new();
        let mut head = head.as_ref();
        while let Some(ref node) = head {
            let value = node.val;
            if value < x {
                smaller.push(value)
            } else {
                larger.push(value);
            }
            head = node.next.as_ref();
        }

        // create a response ListNode
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut current = res.as_mut().unwrap();

        for val in smaller {
            let node = ListNode::new(val);
            current.next = Some(Box::new(node));
            current = current.next.as_mut().unwrap();
        }
        for val in larger {
            let node = ListNode::new(val);
            current.next = Some(Box::new(node));
            current = current.next.as_mut().unwrap();
        }
        res.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 2, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
            3,
        );
        let except = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        let output = Solution::partition(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
