use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            let v2 = target - *v;
            if let Some(i2) = map.get(&v2) {
                return vec![*i2 as i32, i as i32];
            }
            map.insert(*v, i as i32);
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::two_sum(vec![1, 2, 2, 5], 4), vec![1, 2]);
    }
}
