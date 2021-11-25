struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut i = 0usize;
        while i < nums.len() {
            let n = nums[i];
            if let Some(pairs) = Solution::two_sum(&nums, i + 1, -n) {
                for mut pair in pairs {
                    pair.push(n);
                    result.push(pair);
                }
                i += 1;
                // if find match, then skip dup start numbers
                while i < nums.len() && nums[i] == nums[i - 1] {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        result
    }

    fn two_sum(nums: &Vec<i32>, start_idx: usize, target: i32) -> Option<Vec<Vec<i32>>> {
        if start_idx >= nums.len() - 1 {
            return None;
        }
        let mut left = start_idx;
        let mut right = nums.len() - 1;
        let mut result: Vec<Vec<i32>> = Vec::new();
        while left < right {
            // DO NOT skip from the beginning, we might skip the numbers we need!,
            // like [0,0,0,1,1] with target = 2, `right` will bypass all but the first 1,
            // but the final result need two 1s!
            // while left < right - 1 && nums[left] == nums[left + 1] {
            //     left += 1;
            // }
            // while right > left + 1 && nums[right as usize] == nums[(right - 1) as usize] {
            //     right -= 1;
            // }

            let sum = nums[left] + nums[right as usize];
            if sum == target {
                result.push(vec![nums[left], nums[right as usize]]);
                left += 1;
                right -= 1;
                // if find match, skip dup start numbers
                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
                while right > left && nums[right as usize] == nums[(right + 1) as usize] {
                    right -= 1;
                }
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            vec![vec![-1, 2, -1], vec![0, 1, -1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
        assert_eq!(
            Vec::new() as Vec<Vec<i32>>,
            Solution::three_sum(vec![-1, 0])
        );
        assert_eq!(Vec::new() as Vec<Vec<i32>>, Solution::three_sum(vec![]));
        assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(vec![-1, 0, 0, 0]));
    }
}
