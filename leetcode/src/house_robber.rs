struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return nums[0].max(nums[1]);
        }

        let mut money = vec![0; nums.len()];
        money[0] = nums[0];
        money[1] = nums[1].max(nums[0]);
        for i in 2..money.len() {
            money[i] = money[i - 1].max(money[i - 2] + nums[i]);
        }

        money[money.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        assert_eq!(2, Solution::rob(vec![1, 2]));
        assert_eq!(1, Solution::rob(vec![1]));
        assert_eq!(0, Solution::rob(vec![]));
    }
}
