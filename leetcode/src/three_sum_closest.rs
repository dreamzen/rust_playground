struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        let mut nums = nums;
        nums.sort();

        let mut result = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right as usize];
                if sum == target {
                    return target;
                } else if sum > target {
                    right -= 1;
                } else {
                    left += 1;
                }
                if (sum - target).abs() < (result - target).abs() {
                    result = sum;
                }
            }
        }
        result
    }
}
