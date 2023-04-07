pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn recurse(n: i32, k: i32, start_index: i32, path: Vec<i32>) -> Vec<Vec<i32>> {
            let _length = path.len() as i32;
            let mut result: Vec<Vec<i32>> = Vec::new();
            if _length >= k {
                result.push(path);
                return result;
            }
            if start_index >= n {
                return result;
            }
            for i in start_index..n {
                let i = i + 1;
                let mut path_: Vec<i32> = path.to_vec();
                path_.push(i);
                result.append(&mut recurse(n, k, i, path_));
            }
            result
        }
        recurse(n, k, 0, Vec::new())
    }
}

fn main() {
    println!("{:?}", Solution::combine(5, 3));
}


