struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        Solution::generate(String::new(), n, n, &mut result);
        result
    }

    fn generate(sub: String, num_left: i32, num_right: i32, result: &mut Vec<String>) {
        if num_left == 0 && num_right == 0 {
            result.push(sub);
            return;
        }
        if num_left == num_right {
            let mut new_sub = sub.clone();
            new_sub.push('(');
            Solution::generate(new_sub, num_left - 1, num_right, result);
        } else if num_left < num_right {
            let mut new_sub = sub.clone();
            new_sub.push(')');
            Solution::generate(new_sub, num_left, num_right - 1, result);
            if num_left > 0 {
                let mut new_sub = sub.clone();
                new_sub.push('(');
                Solution::generate(new_sub, num_left - 1, num_right, result);
            }
        } else {
            panic!("should never happen!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(vec![String::from("()")], Solution::generate_parenthesis(1));
        assert_eq!(
            vec![String::from("()()"), String::from("(())")],
            Solution::generate_parenthesis(2)
        );
        assert_eq!(vec![String::from("")], Solution::generate_parenthesis(0));
    }
}
