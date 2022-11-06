pub struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // *nums.iter().min().unwrap()
        // robust
        let mut res = nums[0];
        for num in nums {
            res = res.min(num);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![1,3,5];
        let except = 1;
        let output = Solution::find_min(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case2() {
        let inputs = vec![2,2,2,0,1];
        let except = 0;
        let output = Solution::find_min(inputs);
        assert_eq!(except, output);
    }
}