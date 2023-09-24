
struct Solution;

struct FUS {
    parent: Vec<usize>
}

impl FUS {
    pub fn new(n: usize) -> FUS {
        return FUS { parent: (0..n).collect() }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        self.parent[parent_x] = parent_y;
    }
}

impl Solution {
    pub fn dist(point1: &Vec<i32>, point2: &Vec<i32>) -> i32 {
        (point1[0] - point2[0]).abs() + (point1[1] - point2[1]).abs()
    }

    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {

        // solution: kruskal algorithm.

        let mut edge_set: Vec<(usize, usize, i32)> = vec![];
        let len = points.len();
        for i in 0..len {
            for j in i + 1..len {
                if i != j {
                    let d = Solution::dist(&points[i], &points[j]);
                    edge_set.push((i, j, d));
                }
            }
        }

        edge_set.sort_by(|e1, e2|e1.2.cmp(&e2.2));
        let mut fus = FUS::new(len);
        let mut ans = 0;
        for edge in edge_set {
            let (v1, v2, w) = edge;
            if fus.find(v1) != fus.find(v2) {
                ans += w;
                fus.union(v1, v2);
            }
        }
        // kruskal algorithm
        ans
    }
}

use std::error::Error;
fn test(input: &str, result: i32) -> Result<(), Box<dyn Error>> {
    let points = serde_json::from_str(input)?;
    assert_eq!(Solution::min_cost_connect_points(points), result);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    test("[[0,0],[2,2],[3,10],[5,2],[7,0]]", 20)?;
    test("[[3,12],[-2,5],[-4,1]]", 18)?;
    Ok(())
}