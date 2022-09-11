// too slow

pub struct Solution {}
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        // list of given candy
        let mut candy = vec![0; ratings.len()];
        // list of the giving order index list
        let mut indexes = (0..ratings.len()).collect::<Vec<usize>>();
        indexes.sort_by_key(|&i| &ratings[i]);

        for &idx in &indexes {
            let mut num = 1;
            // check left
            if idx>0 && ratings[idx]>ratings[idx-1] {
                num = num.max(candy[idx-1]+1);
            }
            // check right
            if idx<indexes.len()-1 && ratings[idx]>ratings[idx+1] {
                num = num.max(candy[idx+1]+1);
            }

            candy[idx] = num;
        }
        candy.iter().sum()
    }

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![1,0,2];
        let except = 5;
        let output = Solution::candy(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![1,2,2];
        let except = 4;
        let output = Solution::candy(inputs);
        assert_eq!(except, output);
    }
}