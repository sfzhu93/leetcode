
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut ind: Vec<i32> = (1..=num_courses).collect();
        let mut visited: Vec<bool> = vec![false; (num_courses + 1) as usize];
        let mut graph: Vec<Vec<i32>> = (1..=num_courses).map(|x|vec![]).collect();
        for pre in prerequisites {
            graph[pre[0]].push(pre[1]);
            ind[pre[1]] += 1;
        }
        let mut queue = VecDeque::default();
        for (i, x) in ind.iter().enumerate() {
            if x == 0 {
                queue.push_back(i);
                visited[i] = true;
            }
        }

        while !queue.is_empty() {
            let head = queue.pop_front();
            for x in graph[head] {
                ind[x] -= 1
            }
        }

        for i in 1..=num_courses {
            
        }
        false
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