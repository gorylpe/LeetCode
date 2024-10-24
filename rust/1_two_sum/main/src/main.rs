pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate().skip(i + 1) {
            if x + y == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    panic!()
}

fn main() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
