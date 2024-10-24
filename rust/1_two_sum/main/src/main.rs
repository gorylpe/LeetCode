
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map = std::collections::HashMap::<&i32, usize>::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(index) = nums_map.get(&complement) {
                return vec![*index as i32, i as i32]
            }
            nums_map.insert(num, i);
        }
        panic!()
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
