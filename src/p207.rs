
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut ind: Vec<i32> = vec![0; n];
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for pre in prerequisites {
            graph[pre[0] as usize].push(pre[1] as usize);
            ind[pre[1] as usize] += 1;
        }
        println!("{:?}", ind);
        let mut queue = VecDeque::default();
        for (i, x) in ind.iter().enumerate() {
            if *x == 0 {
                queue.push_back(i);
            }
        }

        while !queue.is_empty() {
            let head = queue.pop_front().unwrap();
            println!("{}", head);
            for x in &graph[head] {
                ind[*x] -= 1;
                if ind[*x] == 0 {
                    queue.push_back(*x);
                }
            }
        }

        for i in 0..n {
            if ind[i as usize] > 0 {
                return false
            }
        }
        true
    }
}

struct Solution;

use std::collections::VecDeque;
use std::error::Error;
fn test(num_courses: i32, input: &str, result: bool) -> Result<(), Box<dyn Error>> {
    let prerequisites = serde_json::from_str(input)?;
    assert_eq!(Solution::can_finish(num_courses, prerequisites), result);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    test(2, "[[1,0]]", true)?;
    test(2, "[[1,0],[0,1]]", false)?;
    Ok(())
}
