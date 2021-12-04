struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut idx = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[idx - 1] {
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        let k = Solution::remove_duplicates(&mut nums);
        println!("k = {}, nums = {:?}", k, nums);
    }
}
