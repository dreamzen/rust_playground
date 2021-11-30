use crate::largest_rectangle_in_histogram::Solution as S;

struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }

        let mut max = 0;
        let mut heights = vec![0; matrix[0].len()];

        for vec in matrix.iter() {
            for (i, c) in vec.iter().enumerate() {
                match c {
                    '1' => heights[i] += 1,
                    _ => heights[i] = 0,
                }
            }
            max = max.max(S::largest_rectangle_area(heights.clone()))
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_rectangle() {
        assert_eq!(
            6,
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ])
        );
        assert_eq!(0, Solution::maximal_rectangle(vec![vec![]]));
        assert_eq!(0, Solution::maximal_rectangle(vec![vec!['0']]));
        assert_eq!(1, Solution::maximal_rectangle(vec![vec!['1']]));
        assert_eq!(0, Solution::maximal_rectangle(vec![vec!['0', '0']]));
    }
}
