struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() <= 1 {
            return intervals;
        }
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut result = Vec::new();
        let (mut left, mut right) = (intervals[0][0], intervals[0][1]);

        for (_, v) in intervals.iter().enumerate() {
            if v[0] <= right {
                right = right.max(v[1]);
            } else {
                result.push(vec![left, right]);
                left = v[0];
                right = v[1];
            }
        }
        result.push(vec![left, right]);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(
            "[[1, 6], [8, 10], [15, 18]]",
            format!(
                "{:?}",
                Solution::merge(vec![vec![1, 3], vec![2, 6], vec![15, 18], vec![8, 10]])
            )
        );
    }
}
