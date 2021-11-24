struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() <= 1 {
            return 0;
        }
        let mut max = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let hl = height[left as usize];
            let hr = height[right as usize];
            max = max.max(hl.min(hr) * (right as i32 - left as i32));
            if hl < hr {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(1, Solution::max_area(vec![1, 1]));
        assert_eq!(16, Solution::max_area(vec![4, 3, 2, 1, 4]));
        assert_eq!(2, Solution::max_area(vec![1, 2, 1]));
        assert_eq!(0, Solution::max_area(vec![]));
        assert_eq!(0, Solution::max_area(vec![1]));
    }
}
