pub struct Solution {}
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        // only need to figure out the local maximum

        // // gloabl maximum solution
        // nums.into_iter().enumerate().max_by(|x, y| {
        //     x.1.cmp(&y.1)
        // }).unwrap().0 as i32

        // binary search for local maximum
        let (mut l, mut r) = (0, nums.len()-1);  // right inculsive
        while l<r {
            let mid = (l+r)/2;
            if nums[mid]>nums[mid+1] {
                r = mid;
            } else {
                l = mid+1;
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![1,2,3,1];
        let except = 2;
        let output = Solution::find_peak_element(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![1,2,1,3,5,6,4];
        let except = 1;
        let output = Solution::find_peak_element(inputs);
        assert_eq!(except, output);
    }
}