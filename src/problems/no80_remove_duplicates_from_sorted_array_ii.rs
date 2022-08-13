/*
ideal:

init:
p=0; i=1; d=f;  // d: is_duplicate

step 0
1 1 1 2 2 3
  p i
d: t

step 1
1 1 1 2 2 3
  p   i
d: t

step 2
1. move p
{
    1 1 1 2 2 3
        p i
    d: t
}

2. if d==true {swap it}
1 1 2 1 2 3
    p i
d: t

3. move i and update d
1 1 2 1 2 3
    p   i
d: f

step 3
1. move p
{
    1 1 2 1 2 3
          p i
    d: f
}

2. if d==false {swap it}
1 1 2 2 1 3
      p i
d: f

3. move i and update d
1 1 2 2 1 3
      p   i
d: f
*/

pub struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut is_duplicate = false;
        let mut pivot = 0;
        let mut idx = 1;

        while idx < nums.len() {
            is_duplicate = if nums[idx] == nums[pivot] {
                if !is_duplicate {
                    pivot += 1;
                    nums.swap(pivot, idx);
                }
                true
            } else {
                pivot += 1;
                nums.swap(pivot, idx);
                false
            };
            idx += 1;
        }
        (pivot + 1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut inputs = vec![1, 1, 1, 2, 2, 3];
        println!("nums: {:?}", inputs);
        let except = 5;
        let output = Solution::remove_duplicates(&mut inputs);
        println!("nums: {:?}", inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let mut inputs = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        println!("nums: {:?}", inputs);
        let except = 7;
        let output = Solution::remove_duplicates(&mut inputs);
        println!("nums: {:?}", inputs);
        assert_eq!(except, output);
    }
}
