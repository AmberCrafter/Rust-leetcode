pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.push(new_interval);
        intervals.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        let mut res: Vec<Vec<i32>> = Vec::new();
        let (mut l, mut r) = (None, None);
        for val in intervals {
            if l.is_none() {
                l = Some(val[0]);
                r = Some(val[1]);
            } else {
                if Some(val[0]) <= l && Some(val[1]) >= r {
                    l = Some(val[0])
                }
                if Some(val[0]) >= l && Some(val[1]) <= r {
                    continue;
                }
                if Some(val[0]) <= r && Some(val[1]) >= r {
                    if Some(val[1]) > r {
                        r = Some(val[1])
                    }
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
        let inputs = ([[1, 3].to_vec(), [6, 9].to_vec()].to_vec(), [2, 5].to_vec());
        let expected = [[1, 5].to_vec(), [6, 9].to_vec()].to_vec();
        let result = Solution::insert(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = (
            [
                [1, 2].to_vec(),
                [3, 5].to_vec(),
                [6, 7].to_vec(),
                [8, 10].to_vec(),
                [12, 16].to_vec(),
            ]
            .to_vec(),
            [4, 8].to_vec(),
        );
        let expected = [[1, 2].to_vec(), [3, 10].to_vec(), [12, 16].to_vec()].to_vec();
        let result = Solution::insert(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }
}
