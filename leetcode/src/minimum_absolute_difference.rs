struct Solution;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort();
        println!("{:?}", arr);
        let mut diff = arr[1] - arr[0];
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] < diff {
                diff = arr[i] - arr[i - 1];
            }
        }
        println!("{}", diff);

        let mut result = Vec::new();
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] == diff {
                result.push(vec![arr[i - 1], arr[i]]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_abs_difference() {
        let result = Solution::minimum_abs_difference(vec![40, 11, 26, 27, -20]);
        assert_eq!("[[26, 27]]", format!("{:?}", result));
    }
}
