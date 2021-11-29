struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        if nums.len() < 4 {
            return result;
        }
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() {
            // skip dup numbers except for the first num
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() {
                // skip dup numbers except for the first num after i
                if j != i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                // search from the
                let mut m = j + 1;
                let mut n = nums.len() - 1;
                while m < n {
                    let sum = nums[i] + nums[j] + nums[m] + nums[n];
                    if sum == target {
                        result.push(vec![nums[i], nums[j], nums[m], nums[n]]);
                        m += 1;
                        n -= 1;
                        while m < n && nums[m] == nums[m - 1] {
                            m += 1;
                        }
                        while m < n && nums[n] == nums[n + 1] {
                            n -= 1;
                        }
                    } else if sum > target {
                        n -= 1;
                    } else {
                        m += 1;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum() {
        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0)
        );
        assert_eq!(
            vec![vec![2, 2, 2, 2]],
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8)
        );
    }
}
