pub struct Solution {}
struct MinStack {
    stack: Vec<(i32, i32)>  // (val, min)
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self { stack: Vec::new() }
    }
    
    fn push(&mut self, val: i32) {
        let min = if let Some(value) = self.stack.last() {
            value.1.min(val)
        } else {
            val
        };
        self.stack.push((val, min));
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(obj.get_min(), -3);
        obj.pop();
        assert_eq!(obj.top(), 0);
        assert_eq!(obj.get_min(), -2);
    }
}