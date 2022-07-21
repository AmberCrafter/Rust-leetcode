pub struct Solution {}
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut n = n;
        let mut m = m;
        while m>0 || n>0 {
            if m==0 && n>0 {
                n-=1;
                nums1[(m+n) as usize] = nums2[(n) as usize];
                continue;
            }

            if n==0 && m>0 {
                m-=1;
                nums1[(m+n) as usize] = nums1[(m) as usize];
                continue;
            }

            nums1[(m+n) as usize] = if nums2[(n-1) as usize]>nums1[(m-1) as usize] {
                n-=1;
                nums2[(n) as usize]
            } else {
                m-=1;
                nums1[(m) as usize]
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut inputs = (vec![1,2,3,0,0,0], 3, vec![2,5,6], 3);
        let except = vec![1,2,2,3,5,6];
        Solution::merge(&mut inputs.0,inputs.1,&mut inputs.2,inputs.3);
        let output = inputs.0;
        assert_eq!(except, output);
    }
}