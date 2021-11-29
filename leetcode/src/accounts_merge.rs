use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        // graph stores connected emails
        // input: [[A, 1, 2, 3], [A, 2, 4]]
        // graph will be: {1: [2], 2: [1, 3, 4], 3: [2], 4: [2]}
        for account in accounts.iter() {
            for w in account[1..].windows(2) {
                graph.entry(w[0].clone()).or_insert(vec![]);
                graph.entry(w[1].clone()).or_insert(vec![]);

                graph.get_mut(&w[0]).unwrap().push(w[1].clone());
                graph.get_mut(&w[1]).unwrap().push(w[0].clone());
            }
        }

        let mut result: Vec<Vec<String>> = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();
        for account in accounts.iter() {
            if !visited.contains(&account[1]) {
                // new email means new person, find all the emails, and insert the name
                let mut one_result = Vec::new();
                // dfs visit the graph to find & insert all the emails connected to the person
                Solution::dfs(&mut one_result, &mut visited, &graph, account[1].clone());
                one_result.sort();
                one_result.insert(0, String::from(&account[0])); // insert name after sorting
                result.push(one_result);
            }
        }

        println!("{:?}", result);

        result
    }

    fn dfs(
        result: &mut Vec<String>,
        visited: &mut HashSet<String>,
        graph: &HashMap<String, Vec<String>>,
        email: String,
    ) {
        if !visited.contains(&email) {
            visited.insert(email.clone());
            result.push(email.clone());
        }
        // dfs all the connected emails
        if let Some(connected_emails) = graph.get(&email) {
            for e in connected_emails.iter() {
                if !visited.contains(e) {
                    visited.insert(e.clone());
                    result.push(e.clone());
                    Solution::dfs(result, visited, graph, e.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounts_merge() {
        let result = Solution::accounts_merge(vec![
            vec![
                String::from("A"),
                String::from("1"),
                String::from("2"),
                String::from("3"),
            ],
            vec![String::from("A"), String::from("2"), String::from("4")],
        ]);

        assert_eq!(
            vec![vec![
                String::from("A"),
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4")
            ]],
            result
        );

        let result = Solution::accounts_merge(vec![vec![String::from("A"), String::from("1")]]);
        assert_eq!(vec![vec![String::from("A"), String::from("1")]], result);
    }
}
