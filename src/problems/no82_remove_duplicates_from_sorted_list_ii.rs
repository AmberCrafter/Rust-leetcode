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

pub struct Solution {}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        // read out all of elements and packaged it after process
        let mut buf = Vec::new();
        while let Some(node) = head {
            buf.push(node.val);
            head = node.next;
        }

        let mut skip_value = i32::MAX;
        let mut node = None;
        for i in (1..buf.len()).rev() {
            if buf[i] != buf[i - 1] && buf[i] != skip_value {
                node = Some(Box::new(ListNode {
                    next: node,
                    val: buf[i],
                }));
            } else {
                skip_value = buf[i];
            }
        }

        // the first element in buf
        match buf.len() {
            0 => node,
            1 => Some(Box::new(ListNode::new(buf[0]))),
            _ => {
                if buf[0] != buf[1] {
                    node = Some(Box::new(ListNode {
                        next: node,
                        val: buf[0],
                    }));
                };
                node
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn gen_listnode(array: Vec<i32>) -> Option<Box<ListNode>> {
        let mut node = None;
        for &v in array.iter().rev() {
            node = Some(Box::new(ListNode { next: node, val: v }));
        }
        node
    }

    #[test]
    fn case1() {
        let inputs = gen_listnode(vec![1, 2, 3, 3, 4, 4, 5]);
        let except = gen_listnode(vec![1, 2, 5]);
        let output = Solution::delete_duplicates(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = gen_listnode(vec![1, 1, 1, 2, 3]);
        let except = gen_listnode(vec![2, 3]);
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
