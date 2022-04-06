use std::collections::btree_set::Intersection;

pub struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a,b| a[0].partial_cmp(&b[0]).unwrap());

        let mut res: Vec<Vec<i32>> = Vec::new();
        let (mut l, mut r) = (None, None);
        for val in intervals {
            if l.is_none() {
                l=Some(val[0]);
                r=Some(val[1]);
            } else {
                if Some(val[0])<=l && Some(val[1])>=r {l=Some(val[0])}
                if Some(val[0])>=l && Some(val[1])<=r {continue}       
                if Some(val[0])<=r && Some(val[1])>=r {
                    if Some(val[1])>r {r=Some(val[1])}
                } else {
                    res.push([l.unwrap(), r.unwrap()].to_vec());
                    l = Some(val[0]);
                    r = Some(val[1]);
                }
            }
        }
        res.push([l.unwrap(), r.unwrap()].to_vec());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = [[1,3].to_vec(),[2,6].to_vec(),[8,10].to_vec(),[15,18].to_vec()].to_vec();
        let expected = [[1,6].to_vec(),[8,10].to_vec(),[15,18].to_vec()].to_vec();
        let result = Solution::merge(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = [[1,4].to_vec(),[4,5].to_vec()].to_vec();
        let expected = [[1,5].to_vec()].to_vec();
        let result = Solution::merge(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_3() {
        let inputs = [[1,4].to_vec(),[0,4].to_vec()].to_vec();
        let expected = [[0,4].to_vec()].to_vec();
        let result = Solution::merge(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_4() {
        let inputs = [[1,4].to_vec(),[0,1].to_vec()].to_vec();
        let expected = [[0,4].to_vec()].to_vec();
        let result = Solution::merge(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_5() {
        let inputs = [[1,4].to_vec(),[2,3].to_vec()].to_vec();
        let expected = [[1,4].to_vec()].to_vec();
        let result = Solution::merge(inputs);

        assert_eq!(expected, result);
    }
}
