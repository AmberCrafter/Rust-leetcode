use std::{rc::Rc, cell::RefCell};


struct LRUCache { 
    map: HashMap<i32, i32>,
    lru: ListNode,
}

struct ListNode {
    key: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new() -> Self {
        Self { key: 0, prev: None, next: None }
    }
}

struct CircleListNode {
    head: Rc<RefCell<ListNode>>,
    tail: Rc<RefCell<ListNode>>,
}

impl CircleListNode {
    pub fn new() -> Self {
        let node = Rc::new(RefCell::new(ListNode::new()));
        Self { head: node.clone(), tail: node.clone() }
    }

    pub fn add(&mut self, key: i32) {
        let node = Rc::new(RefCell::new(ListNode {
            key, 
            prev: Some(self.tail.clone()),
            next: Some(self.head.clone())
        }));
        self.tail.borrow_mut().next = Some(node);
        self.tail = node;
    }

    pub fn remove(&self, target: Rc<RefCell<ListNode>>) {
        let prev = target.borrow().prev.clone();
        let next = target.borrow().next.clone();
        prev.as_ref().unwrap().borrow_mut().next = next;
        next.as_ref().unwrap().borrow_mut().prev = prev;
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        
    }
    
    fn get(&self, key: i32) -> i32 {
        
    }
    
    fn put(&self, key: i32, value: i32) {
        
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

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