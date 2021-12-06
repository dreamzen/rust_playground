struct Solution;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut odd = 0;
        let mut even = 0;
        for (i, v) in position.iter().enumerate() {
            if v % 2 == 0 {
                odd += 1;
            } else {
                even += 1;
            }
        }
        odd.min(even)
    }
}
