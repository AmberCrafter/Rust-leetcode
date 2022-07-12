pub struct Solution {}
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // dp: use two table
        // 1st table figure out maxiumn valied height in range [i, j]
        // 2ed table record maxiumn area and output the last value

        let lenght = heights.len() + 1;

        let mut height_table = vec![vec![0; lenght]; lenght];
        let mut area_table = vec![vec![0; lenght]; lenght];

        for i in 1..lenght {
            height_table[i][i] = heights[i - 1];
            height_table[i][0] = heights[i - 1];
            height_table[0][i] = heights[i - 1];
        }

        for r in 1..lenght {
            for c in r + 1..lenght {
                height_table[r][c] = heights[c - 1]
                    .min(height_table[r][c - 1]);
            }
        }

        for r in 1..lenght {
            for c in r..lenght {
                area_table[r][c] = (height_table[r][c] * (c - r + 1) as i32)
                    .max(area_table[r - 1][c])
                    .max(area_table[r][c - 1]);
            }
        }

        area_table.last().unwrap().last().unwrap().to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![2, 1, 5, 6, 2, 3];
        let except = 10;
        let output = Solution::largest_rectangle_area(inputs);
        assert_eq!(except, output);
    }
}
