struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices = (0..nums.len()).collect::<Vec<_>>();
        indices.sort_unstable_by(|&i, &j| nums[i].cmp(&nums[j]));
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i != j {
            let sum = nums[indices[i]] + nums[indices[j]];
            if sum > target {
                j -= 1;
            } else if sum < target {
                i += 1;
            } else {
                return vec![indices[i] as i32, indices[j] as i32];
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
