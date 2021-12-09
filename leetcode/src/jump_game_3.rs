struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visited = vec![false; arr.len()];
        return Self::visit(&arr, start, &mut visited);
    }

    fn visit(arr: &Vec<i32>, start: i32, visited: &mut Vec<bool>) -> bool {
        if start < 0 || start >= arr.len() as i32 {
            return false;
        }
        if arr[start as usize] == 0 {
            return true;
        }
        if visited[start as usize] == true {
            return false;
        }
        visited[start as usize] = true;
        Self::visit(arr, start + arr[start as usize], visited)
            || Self::visit(arr, start - arr[start as usize], visited)
    }
}
