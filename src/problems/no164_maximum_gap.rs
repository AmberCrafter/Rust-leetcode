use std::collections::BTreeSet;

pub struct Solution {}
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        // requirement:
        //   time: O(n)
        //   space: O(n)

        // step: sort -> find gap
        let mut tree = BTreeSet::new();
        for num in nums {
            tree.insert(num);
        }

        let mut res = 0;
        let mut tree_iter = tree.into_iter();
        let mut pre = tree_iter.next().unwrap();
        while let Some(cur) = tree_iter.next() {
            res = res.max(cur-pre);
            pre = cur;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![3,6,9,1,5,7,9,1,5,6];
        let except = 2;
        let output = Solution::maximum_gap(inputs);
        assert_eq!(except, output);
    }
}