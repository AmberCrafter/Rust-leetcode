pub struct Solution {}
impl Solution {
    // best solution
    // ref: https://leetcode.com/problems/maximum-product-subarray/discuss/1613617/Rust
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut current_max = 1;
        let mut current_min = 1;
        for num in nums {
            let max_tmp = current_max;
            current_max = num.max(num*max_tmp).max(num*current_min);
            current_min = num.min(num*max_tmp).min(num*current_min);
            result = result.max(current_max);
        }
        result
    }
    // pub fn max_product(nums: Vec<i32>) -> i32 {
    //     let mut nums = nums;
    //     let mut res = i32::MIN;
    //     for i in 0..nums.len() {
    //         for j in 0..i {
    //             nums[j] = nums[j] * nums[i];
    //             res = res.max(nums[j]);
    //         }
    //         res = res.max(nums[i]);
    //     }
    //     res
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![2,3,-2,4];
        let except = 6;
        let output = Solution::max_product(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![-2, 0, -1];
        let except = 6;
        let output = Solution::max_product(inputs);
        assert_eq!(except, output);
    }
}