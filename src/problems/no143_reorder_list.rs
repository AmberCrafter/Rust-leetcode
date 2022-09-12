use std::collections::VecDeque;

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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut buf = VecDeque::new();

        if let Some(root) = head.as_mut() {
            let cur = root.next.take();
            buf.push_back(cur);
            while let Some(node) = buf.back_mut().unwrap() {
                let next = node.next.take();
                if next.is_some() {
                    buf.push_back(next);
                } else {
                    break;
                }
            }
        }

        let mut cur = head.as_mut().unwrap();
        while buf.len()>0 {
            if let Some(node) = buf.pop_back() {
                cur.next = node;
                if buf.len()>0 {
                    cur = cur.next.as_mut().unwrap();
                } else {
                    break;
                }
            }
            if let Some(node) = buf.pop_front() {
                cur.next = node;
                cur = cur.next.as_mut().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut inputs = ListNode::gen(vec![1,2,3,4]);
        let except = ListNode::gen(vec![1,4,2,3]);
        Solution::reorder_list(&mut inputs);
        assert_eq!(except, inputs);
    }

    #[test]
    fn case2() {
        let mut inputs = ListNode::gen(vec![1,2,3,4,5]);
        let except = ListNode::gen(vec![1,5,2,4,3]);
        Solution::reorder_list(&mut inputs);
        assert_eq!(except, inputs);
    }
}

impl ListNode {
    fn gen(arr: Vec<i32>) -> Option<Box<Self>> {
        let mut root = Some(Box::new(ListNode::new(0)));
        let mut cur = root.as_mut().unwrap();
        for val in arr {
            cur.next.replace(Box::new(ListNode::new(val)));
            cur = cur.next.as_mut().unwrap();
        }
        root.unwrap().next
    }
}