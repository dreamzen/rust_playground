struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        if n == 1 {
            return true;
        }

        let mut n = n;
        while n != 1 {
            if n % 2 != 0 {
                return false;
            }
            n = n / 2;
        }
        true
    }
}
