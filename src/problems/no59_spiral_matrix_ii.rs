pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = (0..n)
            .map(|_| (0..n).map(|_| 0).collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();
        let (mut l, mut r) = (0, n as usize - 1);
        let (mut t, mut b) = (0, n as usize - 1);
        let mut value = 1;

        while r >= l && b >= t {
            for i in l..=r {
                res[t][i] = value;
                value += 1;
            }
            t += 1;

            for i in t..=b {
                res[i][r] = value;
                value += 1;
            }
            if r == 0 {
                break;
            }
            r -= 1;

            for i in (l..=r).rev() {
                res[b][i] = value;
                value += 1;
            }
            if b == 0 {
                break;
            }
            b -= 1;

            for i in (t..=b).rev() {
                res[i][l] = value;
                value += 1;
            }
            l += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = 3;
        let expected = [[1, 2, 3].to_vec(), [8, 9, 4].to_vec(), [7, 6, 5].to_vec()].to_vec();
        let result = Solution::generate_matrix(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = 1;
        let expected = [[1].to_vec()].to_vec();
        let result = Solution::generate_matrix(inputs);

        assert_eq!(expected, result);
    }
}
