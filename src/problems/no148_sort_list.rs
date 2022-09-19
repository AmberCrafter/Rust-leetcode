pub struct Solution {}
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn insertion_sort(arr: &mut Vec<Option<Box<ListNode>>>) {
            // largest ... smallest
            let mut cur = 1;
            let mut right = 1;
            while right<arr.len() {
                while arr[cur].as_ref().unwrap().val>arr[cur-1].as_ref().unwrap().val {
                    arr.swap(cur-1, cur);
                    cur-=1;
                }
                right+=1;
                cur = right;
            }
        }
        
        // distructure the listnode
        let mut buf = Vec::new();
        buf.push(head);
        loop {
            let next = buf.last().unwrap().next.take();
            if next.is_none() {
                break;
            }
            buf.push(next);
        }

        // sort
        insertion_sort(&mut buf);

        // generate list
        let mut root = buf.pop().unwrap();
        let mut cur = root.as_mut().unwrap();

        while let Some(node) = buf.pop() {
            cur.next = node;
            cur = cur.next.as_mut().unwrap();
        }
        root
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = None;
        let except = None;
        let output = None;
        assert_eq!(except, output);
    }
}