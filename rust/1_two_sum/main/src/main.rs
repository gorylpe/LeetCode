struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_i: Vec<_> = nums.iter().enumerate().collect();
        nums_i.sort_by(|a, b| a.1.cmp(&b.1));
        let mut i = 0;
        let mut j = nums_i.len() - 1;
        while i != j {
            let sum = nums_i[i].1 + nums_i[j].1;
            if sum > target {
                j -= 1;
            } else if sum < target {
                i += 1;
            } else {
                return vec![nums_i[i].0 as i32, nums_i[j].0 as i32];
            }
        }
        panic!("no two sums found")
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
