pub struct Solution {}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let left = (left-1) as usize;
        let right = (right-1) as usize;
        // unpack linkedlist
        let mut array = Vec::new();
        let mut head = head.as_ref();
        while let Some(node) = head {
            array.push(node.val);
            head = node.next.as_ref();
        }

        let mut res = Some(Box::new(ListNode::new(0)));
        let mut node = res.as_mut().unwrap();
        for i in 0..array.len() {
            if i>=left && i<=right {
                node.next = Some(Box::new(ListNode::new(array[right-(i-left)])));
            } else {
                node.next = Some(Box::new(ListNode::new(array[i])));
            }
            node = node.next.as_mut().unwrap();
        }
        res.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (Some(Box::new(ListNode {
            val: 1, 
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None
                        }))
                    }))
                }))
            }))
        })), 2,4);
        let except = Some(Box::new(ListNode {
            val: 1, 
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None
                        }))
                    }))
                }))
            }))
        }));;
        let output = Solution::reverse_between(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }
}