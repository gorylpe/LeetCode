struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == i32::MIN {
            return 0;
        }
        if x == 0 {
            return 0;
        }
        let mut result = 0;
        let mut quot = x;
        loop {
            let rem = quot % 10;
            quot = quot / 10;

            result += rem;
            if quot == 0 {
                break;
            }
            if result > i32::MAX / 10 || result < i32::MIN / 10 {
                return 0;
            }
            result *= 10;
        }
        result
    }
}

pub fn test() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(1), 1);
    assert_eq!(Solution::reverse(10), 1);
    assert_eq!(Solution::reverse(-10), -1);
    assert_eq!(Solution::reverse(i32::MIN), 0);
    assert_eq!(Solution::reverse(i32::MAX), 0);
}