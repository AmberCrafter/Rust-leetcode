pub struct Solution {}
impl Solution {
    fn plus(stack: &mut Vec<i32>) {
        let tmp1 = stack.pop().unwrap();
        let tmp2 = stack.pop().unwrap();
        stack.push(tmp1 + tmp2);
    }
    fn minus(stack: &mut Vec<i32>) {
        let tmp1 = stack.pop().unwrap();
        let tmp2 = stack.pop().unwrap();
        stack.push(tmp2 - tmp1);
    }
    fn power(stack: &mut Vec<i32>) {
        let tmp1 = stack.pop().unwrap();
        let tmp2 = stack.pop().unwrap();
        stack.push(tmp1 * tmp2);
    }
    fn divide(stack: &mut Vec<i32>) {
        let tmp1 = stack.pop().unwrap();
        let tmp2 = stack.pop().unwrap();
        stack.push(tmp2 / tmp1);;
    }

    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        tokens.reverse();
        while let Some(next) = tokens.pop() {
            match next.as_str() {
                "+" => Self::plus(&mut stack),
                "-" => Self::minus(&mut stack),
                "*" => Self::power(&mut stack),
                "/" => Self::divide(&mut stack),
                next => stack.push(next.parse::<i32>().unwrap())
            }
        }
        if stack.is_empty() {
            0
        } else {
            stack.pop().unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec!["2".to_string(),"1".to_string(),"+".to_string(),"3".to_string(),"*".to_string()];
        let except = 9;
        let output = Solution::eval_rpn(inputs);
        assert_eq!(except,output);
    }
    #[test]
    fn case2() {
        let inputs = vec!["4".to_string(),"13".to_string(),"5".to_string(),"/".to_string(),"+".to_string()];
        let except = 6;
        let output = Solution::eval_rpn(inputs);
        assert_eq!(except,output);
    }
    #[test]
    fn case3() {
        let inputs = vec!["10".to_string(),"6".to_string(),"9".to_string(),"3".to_string(),"+".to_string(),"-11".to_string(),"*".to_string(),"/".to_string(),"*".to_string(),"17".to_string(),"+".to_string(),"5".to_string(),"+".to_string()];
        let except = 22;
        let output = Solution::eval_rpn(inputs);
        assert_eq!(except,output);
    }
    #[test]
    fn case4() {
        let inputs = vec!["3".to_string(),"11".to_string(),"+".to_string(),"5".to_string(),"-".to_string()];
        let except = 9;
        let output = Solution::eval_rpn(inputs);
        assert_eq!(except,output);
    }
}