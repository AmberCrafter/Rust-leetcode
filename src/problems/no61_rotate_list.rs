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
        // use the Option.take() method to achive the result.
        // when use the Option.take(), it will take the content and leave None
        // ex.
        /*
        if there have a node like this
        node1=ListNode {val, next: {ListNode{...}}}

        and run this code
        > node2 = node1.next.take();

        then we can get the result like
        node1=ListNode {val, next: None}
        node2=ListNode {val, next: ListNode{...}}
        and move the ownership
        */

        // Note. this performace may worse than
        //      store the element in Vec<i32>
        //      and re-generate the new ListNode to return
        //  because we need to create a larger space ( Vec<Option<Box<T>>> ) and fat pointer move.
        //
        // We can use below code to check the ListNode address witch wrapped with Box.
        // println!("{:p}", &**head.as_ref().unwrap());

        // main code with address check

        // println!("{:p}", &**head.as_ref().unwrap());
        if k == 0 {
            return head;
        }
        let mut head = head;
        // println!("{:p}", &**head.as_ref().unwrap());
        let mut arr: Vec<Option<Box<ListNode>>> = Vec::new();

        while head.is_some() {
            let next = head.as_mut().unwrap().next.take();
            arr.push(head);
            head = next;
        }

        let length = arr.len();
        if length == 0 {
            return None;
        }

        let sp = length - (k as usize % length);
        let mut res = arr[(sp + length - 1) % length].take();
        for i in 1..length {
            let mut tmp = arr[(sp + length - i - 1) % length].take();
            tmp.as_mut().unwrap().next = res;
            res = tmp;
        }

        // println!("{:p}", &**res.as_ref().unwrap());
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
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        });
        let inputs = (listnode, 4);
        let expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));
        let result = Solution::rotate_right(Some(inputs.0), inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_3() {
        let listnode = Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        });
        let inputs = (listnode, 0);
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        let result = Solution::rotate_right(Some(inputs.0), inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_4() {
        let listnode = Box::new(ListNode { val: 1, next: None });
        let inputs = (listnode, 1);
        let expected = Some(Box::new(ListNode { val: 1, next: None }));
        let result = Solution::rotate_right(Some(inputs.0), inputs.1);

        assert_eq!(expected, result);
    }
}
