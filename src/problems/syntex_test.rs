
#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    pub fn new(val: i32) -> Option<Box<Self>> {
        Some(Box::new(ListNode { val, next: None }))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let node = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None
                }))
            }))
        }));

        // reverse the node 
        let mut node = node;
        let next = node.as_mut().unwrap().next.take();

        // let new_node = node;
        println!("node: {:?}", node);
        println!("next: {:?}", next);
        println!("{}",5%7);
        println!("{}",15%7);
        println!("{}",35%7);
        println!("{}",85%7);
    }
}
