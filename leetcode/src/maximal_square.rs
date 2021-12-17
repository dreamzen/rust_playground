struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut edges: Vec<Vec<u32>> = Vec::new();
        for i in 0..matrix.len() {
            edges.push(vec![0; matrix[i].len()]);
            for j in 0..matrix[i].len() {
                match matrix[i][j] {
                    '0' => edges[i][j] = 0,
                    _ => edges[i][j] = 1,
                }
            }
        }
        let mut result = 0;
        for i in 0..edges.len() {
            result = result.max(edges[i][0]);
        }
        for i in 0..edges[0].len() {
            result = result.max(edges[0][i]);
        }
        for i in 1..edges.len() {
            for j in 1..edges[i].len() {
                if edges[i][j] == 0 {
                    continue;
                }
                edges[i][j] = 1 + edges[i - 1][j]
                    .min(edges[i][j - 1])
                    .min(edges[i - 1][j - 1]);
                result = result.max(edges[i][j]);
            }
        }
        (result * result) as i32
    }
}
