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

pub struct Solution {}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        // read out all of elements and packaged it after process
        let mut buf = Vec::new();
        while let Some(node) = head {
            if let Some(last) = buf.last() {
                if last!=&node.val {
                    buf.push(node.val);
                }
            } else {
                buf.push(node.val);
            }
            head = node.next;
        }

        let mut node = None;
        for &v in buf.iter().rev() {
            node = Some(Box::new(ListNode {
                next: node,
                val: v
            }));
        }
        node
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn gen_listnode(array: Vec<i32>) -> Option<Box<ListNode>> {
        let mut node = None;
        for &v in array.iter().rev() {
            node = Some(Box::new(
                ListNode {
                    next: node,
                    val: v
                }
            ));
        };
        node
    }

    #[test]
    fn case1() {
        let inputs = gen_listnode(vec![1,1,2]);
        let except = gen_listnode(vec![1,2]);
        let output = Solution::delete_duplicates(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = gen_listnode(vec![1,1,2,3,3]);
        let except = gen_listnode(vec![1,2,3]);
        let output = Solution::delete_duplicates(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = None;
        let except = None;
        let output = Solution::delete_duplicates(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = gen_listnode(vec![1]);
        let except = gen_listnode(vec![1]);
        let output = Solution::delete_duplicates(inputs);
        assert_eq!(except, output);
    }
}