pub struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut is_duplicate = false;
        let mut pivot = 0;
        let mut idx = 1;

        while idx<nums.len() {
            is_duplicate = if nums[idx]==nums[pivot] {
                if !is_duplicate {
                    pivot+=1;
                    nums.swap(pivot, idx);
                }
                true
            } else {
                pivot+=1;
                nums.swap(pivot, idx);
                false
            };
            idx+=1;
        }
        (pivot+1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut inputs = vec![1,1,1,2,2,3];
        println!("nums: {:?}", inputs);
        let except = 5;
        let output = Solution::remove_duplicates(&mut inputs);
        println!("nums: {:?}", inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let mut inputs = vec![0,0,1,1,1,1,2,3,3];
        println!("nums: {:?}", inputs);
        let except = 7;
        let output = Solution::remove_duplicates(&mut inputs);
        println!("nums: {:?}", inputs);
        assert_eq!(except, output);
    }
}


