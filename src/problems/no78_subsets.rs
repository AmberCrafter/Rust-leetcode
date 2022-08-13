pub struct Solution {}
impl Solution {
    fn append(result: &mut Vec<Vec<i32>>, buf: &Vec<i32>, nums: &[i32]) {
        result.push(buf.clone());
        for (i, &e) in nums.iter().enumerate() {
            let mut buf = buf.clone();
            buf.push(e);
            Solution::append(result, &buf, &nums[i + 1..])
        }
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];
        for (i, &e) in nums.iter().enumerate() {
            let buf = vec![e];
            Solution::append(&mut result, &buf, &nums[i + 1..]);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![1, 2, 3];
        let except = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        let output = Solution::subsets(inputs);
        let mut except = except;
        except.sort();
        let mut output = output;
        output.sort();
        assert_eq!(except, output);
    }
}
