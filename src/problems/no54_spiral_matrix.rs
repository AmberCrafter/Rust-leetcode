pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let (mut l, mut h, mut r) = (0, 0, matrix[0].len() - 1);
        let (mut t, mut v, mut b) = (0, 0, matrix.len() - 1);

        let mut taggle = false;
        let mut outofbound = false;
        let mut step = 0;
        // res.push(matrix[0][0]);

        while (r >= l) && (b >= t) && (!outofbound) {
            match step % 4 {
                0 => {
                    for _i in l..=r {
                        if taggle {
                            h += 1
                        } else {
                            taggle = true
                        }
                        res.push(matrix[v][h]);
                    }
                    t += 1;
                }

                1 => {
                    for _i in t..=b {
                        v += 1;
                        res.push(matrix[v][h]);
                    }
                    if r > 0 {
                        r -= 1
                    } else {
                        outofbound = true
                    }
                }

                2 => {
                    for _i in l..=r {
                        if h > 0 {
                            h -= 1
                        } else {
                            outofbound = true
                        }
                        res.push(matrix[v][h]);
                    }
                    if b > 0 {
                        b -= 1
                    } else {
                        outofbound = true
                    }
                }

                3 => {
                    for _i in t..=b {
                        if v > 0 {
                            v -= 1
                        } else {
                            outofbound = true
                        }
                        res.push(matrix[v][h]);
                    }
                    l += 1;
                }

                _ => {
                    panic!("Error")
                }
            }
            step += 1;
            // println!("{:?}", res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        let expected = [1, 2, 3, 6, 9, 8, 7, 4, 5].to_vec();
        let result = Solution::spiral_order(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = [
            [1, 2, 3, 4].to_vec(),
            [5, 6, 7, 8].to_vec(),
            [9, 10, 11, 12].to_vec(),
        ]
        .to_vec();
        let expected = [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7].to_vec();
        let result = Solution::spiral_order(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_3() {
        let inputs = [[3].to_vec(), [2].to_vec()].to_vec();
        let expected = [3, 2].to_vec();
        let result = Solution::spiral_order(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_4() {
        let inputs = [[7].to_vec(), [9].to_vec(), [6].to_vec()].to_vec();
        let expected = [7, 9, 6].to_vec();
        let result = Solution::spiral_order(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_5() {
        let inputs = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        let expected = [1, 2, 4, 3].to_vec();
        let result = Solution::spiral_order(inputs);

        assert_eq!(expected, result);
    }
}
