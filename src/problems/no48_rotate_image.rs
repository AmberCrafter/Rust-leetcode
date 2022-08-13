pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // for rotate 2d square matrix by 90 degree
        let length = matrix.len();
        for i in 0..=length / 2 {
            for j in i..length - i - 1 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[length - 1 - j][i];
                matrix[length - 1 - j][i] = matrix[length - i - 1][length - 1 - j];
                matrix[length - i - 1][length - 1 - j] = matrix[j][length - 1 - i];
                matrix[j][length - 1 - i] = tmp;
                // println!("{:?}", matrix);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut inputs = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = [[7, 4, 1], [8, 5, 2], [9, 6, 3]].to_vec();
        Solution::rotate(&mut inputs);

        assert_eq!(inputs, expected);
    }
}
