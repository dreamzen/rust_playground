struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // key point to calculate the area: for each of the bar, find the left&right boundary.
        // use stack to store the bars in `ascending` order of height.
        // when we visit all the bars in `heights` from left to right,
        // if height of r >= l => push into the stack
        // else means we meet the right boundary of the previous bar.
        // Then how to figure out which is the left boundary? The answer is:
        // For each of the bar (B) in the stack, previous bar is the left boundary (excluded),
        // because if there's any bar in the left shorter than B, then it must be pushed into stack
        // before B.

        let mut heights = heights;
        heights.push(0); // add 0 to the end, so that we can force pop all the bars
        let mut stack = Vec::new(); // store indexes

        let mut right = 0usize;
        let mut max = 0;

        while right < heights.len() {
            // if stack is empty, or current bar is higher/equal, then push to stack
            if stack.len() == 0 || heights[right] >= heights[stack[stack.len() - 1] as usize] {
                stack.push(right as i32);
                right += 1;
            } else {
                // `right` is the right boundary
                let mut h = heights[stack.pop().unwrap() as usize]; // take h
                let mut left = -1; // calculate the left boundary
                if stack.len() > 0 {
                    left = stack[stack.len() - 1];
                }
                max = max.max(h * (right as i32 - left - 1));
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle() {
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
        assert_eq!(4, Solution::largest_rectangle_area(vec![2, 4]));
        assert_eq!(0, Solution::largest_rectangle_area(vec![0]));
        assert_eq!(1, Solution::largest_rectangle_area(vec![1]));
    }
}
