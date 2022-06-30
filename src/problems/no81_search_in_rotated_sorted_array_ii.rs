pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums[0]< target {
            for v in nums.iter() {
                if v==&target {return true}
            }
        } else {
            for v in nums.iter().rev() {
                if v==&target {return true}
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (vec![2, 5, 6, 0, 0, 1, 2], 0);
        let except = true;
        let output = Solution::search(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (vec![2, 5, 6, 0, 0, 1, 2], 3);
        let except = false;
        let output = Solution::search(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (vec![1,0,1,1,1], 0);
        let except = true;
        let output = Solution::search(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
